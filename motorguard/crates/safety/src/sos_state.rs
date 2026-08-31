use serde::{Deserialize, Serialize};

/// Client-side SOS state machine states.
/// The actual server state is persisted in `sos_events.status`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SosState {
    /// No active SOS.
    Idle,
    /// Countdown running — user can still cancel.
    Countdown { seconds_remaining: u32 },
    /// SOS dispatched and contacts notified.
    Active { event_id: String },
    /// SOS cancelled by user before dispatch.
    Cancelled,
    /// SOS resolved (user is safe / assisted).
    Resolved,
}

impl SosState {
    pub fn is_active(&self) -> bool {
        matches!(self, SosState::Active { .. })
    }

    pub fn is_countdown(&self) -> bool {
        matches!(self, SosState::Countdown { .. })
    }
}

/// Default countdown duration in seconds (matching the UI design: 10s).
pub const DEFAULT_COUNTDOWN_SECONDS: u32 = 10;
