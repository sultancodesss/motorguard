use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, Query, State, WebSocketUpgrade,
    },
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use motorguard_core::types::GroupId;
use motorguard_groups::messages::{ClientMessage, ServerMessage};
use serde::Deserialize;
use std::str::FromStr;
use tracing::{debug, info, warn};

use crate::state::AppState;

#[derive(Deserialize)]
pub struct WsQuery {
    pub token: String,
}

/// GET /ws/groups/:group_id?token=<jwt>
///
/// Upgrades to a WebSocket connection for real-time group location sharing.
pub async fn group_ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(group_id_str): Path<String>,
    Query(q): Query<WsQuery>,
) -> Response {
    // Validate token before accepting the upgrade
    let claims = match state.jwt.verify_access_token(&q.token) {
        Ok(c) => c,
        Err(_) => {
            return axum::response::IntoResponse::into_response((
                axum::http::StatusCode::UNAUTHORIZED,
                "Invalid token",
            ));
        }
    };

    let user_id = match claims.user_id() {
        Ok(id) => id,
        Err(_) => {
            return axum::response::IntoResponse::into_response((
                axum::http::StatusCode::UNAUTHORIZED,
                "Invalid token claims",
            ));
        }
    };

    let group_id = match GroupId::from_str(&group_id_str) {
        Ok(id) => id,
        Err(_) => {
            return axum::response::IntoResponse::into_response((
                axum::http::StatusCode::BAD_REQUEST,
                "Invalid group id",
            ));
        }
    };

    // Check membership
    if state.groups.assert_member(group_id, user_id).await.is_err() {
        return axum::response::IntoResponse::into_response((
            axum::http::StatusCode::FORBIDDEN,
            "Not a member of this group",
        ));
    }

    let group_id_str = group_id_str.clone();
    ws.on_upgrade(move |socket| handle_socket(socket, state, group_id_str, user_id))
}

async fn handle_socket(
    socket: WebSocket,
    state: AppState,
    group_id: String,
    user_id: motorguard_core::types::UserId,
) {
    let (mut sender, mut receiver) = socket.split();

    // Subscribe to the group broadcast channel
    let mut rx = state.channels.subscribe(&group_id);

    // Get user name for announcements
    let user_name = {
        use motorguard_database::UserRepository;
        UserRepository::new(&state.db)
            .find_by_id(user_id)
            .await
            .map(|u| u.display_name().to_string())
            .unwrap_or_else(|_| "Rider".to_string())
    };

    // Announce arrival
    state.channels.broadcast(
        &group_id,
        &ServerMessage::MemberJoined {
            user_id: user_id.to_string(),
            name: user_name.clone(),
        },
    );
    info!("WS: {} joined group {}", user_name, group_id);

    // Spawn task: forward broadcasts to this client
    let mut send_task = tokio::spawn(async move {
        while let Ok(json) = rx.recv().await {
            if sender.send(Message::Text(json)).await.is_err() {
                break;
            }
        }
    });

    // Receive messages from this client
    let channels = state.channels.clone();
    let group_id_recv = group_id.clone();
    let user_name_recv = user_name.clone();
    let user_id_str = user_id.to_string();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    match serde_json::from_str::<ClientMessage>(&text) {
                        Ok(ClientMessage::LocationUpdate {
                            latitude,
                            longitude,
                            speed,
                            heading,
                            timestamp,
                        }) => {
                            debug!(
                                "WS location from {}: ({:.4}, {:.4})",
                                user_name_recv, latitude, longitude
                            );
                            channels.broadcast(
                                &group_id_recv,
                                &ServerMessage::MemberLocation {
                                    user_id: user_id_str.clone(),
                                    name: user_name_recv.clone(),
                                    latitude,
                                    longitude,
                                    speed,
                                    heading,
                                    timestamp,
                                },
                            );
                        }
                        Ok(ClientMessage::Ping) => {
                            // Pong is sent below — handled by the send_task
                            // after we insert it into the channel. Instead
                            // send directly (fire-and-forget pattern):
                            // We can't access sender here (moved), so we
                            // broadcast a special pong only to this user.
                            // Simplified: just ignore pings from clients.
                        }
                        Err(e) => {
                            warn!("WS: invalid message from {}: {}", user_name_recv, e);
                        }
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    });

    // Wait for either task to finish (connection closed)
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }

    // Announce departure
    state.channels.broadcast(
        &group_id,
        &ServerMessage::MemberLeft {
            user_id: user_id.to_string(),
        },
    );
    state.channels.remove_if_empty(&group_id);
    info!("WS: {} left group {}", user_name, group_id);
}
