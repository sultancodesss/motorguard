use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Messages sent FROM the mobile client TO the server over WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    /// Regular GPS location update.
    LocationUpdate {
        latitude: f64,
        longitude: f64,
        speed: f64,
        heading: Option<f64>,
        timestamp: DateTime<Utc>,
    },
    /// Keepalive ping.
    Ping,
}

/// Messages sent FROM the server TO mobile clients over WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    /// Another group member's location update.
    MemberLocation {
        user_id: String,
        name: String,
        latitude: f64,
        longitude: f64,
        speed: f64,
        heading: Option<f64>,
        timestamp: DateTime<Utc>,
    },
    /// A member connected to the group channel.
    MemberJoined {
        user_id: String,
        name: String,
    },
    /// A member disconnected from the group channel.
    MemberLeft {
        user_id: String,
    },
    /// Response to a Ping.
    Pong,
    /// Server-side error that the client should display.
    Error {
        code: String,
        message: String,
    },
}

impl ServerMessage {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| r#"{"type":"error"}"#.to_string())
    }
}
