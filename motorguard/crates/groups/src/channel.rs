use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tokio::sync::broadcast;
use tracing::{debug, info};

use crate::messages::ServerMessage;

/// Capacity of each group broadcast channel.
const CHANNEL_CAPACITY: usize = 64;

/// A sender handle for a single group's broadcast channel.
type GroupSender = broadcast::Sender<String>;

/// Registry of all active group WebSocket channels.
///
/// Each group gets its own broadcast channel. When the last member disconnects
/// the channel is dropped automatically (no active senders → no subscribers).
#[derive(Clone, Default)]
pub struct GroupChannelRegistry {
    channels: Arc<RwLock<HashMap<String, GroupSender>>>,
}

impl GroupChannelRegistry {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Subscribe to a group's broadcast channel.
    /// Creates the channel if it does not yet exist.
    pub fn subscribe(&self, group_id: &str) -> broadcast::Receiver<String> {
        let mut map = self.channels.write().unwrap();
        if let Some(tx) = map.get(group_id) {
            debug!("Subscribing to existing channel for group {}", group_id);
            tx.subscribe()
        } else {
            info!("Creating new channel for group {}", group_id);
            let (tx, rx) = broadcast::channel(CHANNEL_CAPACITY);
            map.insert(group_id.to_string(), tx);
            rx
        }
    }

    /// Get a sender for the given group (creates channel if needed).
    pub fn sender(&self, group_id: &str) -> GroupSender {
        let mut map = self.channels.write().unwrap();
        if let Some(tx) = map.get(group_id) {
            tx.clone()
        } else {
            let (tx, _) = broadcast::channel(CHANNEL_CAPACITY);
            map.insert(group_id.to_string(), tx.clone());
            tx
        }
    }

    /// Broadcast a message to all subscribers of a group.
    /// Returns number of receivers that received the message.
    pub fn broadcast(&self, group_id: &str, msg: &ServerMessage) -> usize {
        let json = msg.to_json();
        let map = self.channels.read().unwrap();
        if let Some(tx) = map.get(group_id) {
            tx.send(json).unwrap_or(0)
        } else {
            0
        }
    }

    /// Remove a stale channel (called when no members remain).
    pub fn remove_if_empty(&self, group_id: &str) {
        let mut map = self.channels.write().unwrap();
        if let Some(tx) = map.get(group_id) {
            if tx.receiver_count() == 0 {
                map.remove(group_id);
                debug!("Removed empty channel for group {}", group_id);
            }
        }
    }
}
