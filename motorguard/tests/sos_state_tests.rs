/// Tests for the SOS state machine and related types.

use motorguard_safety::sos_state::{SosState, DEFAULT_COUNTDOWN_SECONDS};

#[test]
fn default_countdown_is_10() {
    assert_eq!(DEFAULT_COUNTDOWN_SECONDS, 10);
}

#[test]
fn idle_state_is_not_active() {
    let state = SosState::Idle;
    assert!(!state.is_active());
}

#[test]
fn countdown_state_is_not_active() {
    let state = SosState::Countdown { seconds_remaining: 5 };
    assert!(!state.is_active());
    assert!(state.is_countdown());
}

#[test]
fn active_state_is_active() {
    let state = SosState::Active { event_id: "test-id".to_string() };
    assert!(state.is_active());
    assert!(!state.is_countdown());
}

#[test]
fn cancelled_state_not_active() {
    let state = SosState::Cancelled;
    assert!(!state.is_active());
    assert!(!state.is_countdown());
}

#[test]
fn resolved_state_not_active() {
    let state = SosState::Resolved;
    assert!(!state.is_active());
}

#[test]
fn countdown_zero_remaining() {
    let state = SosState::Countdown { seconds_remaining: 0 };
    assert!(state.is_countdown());
}

#[test]
fn state_serialises_to_snake_case() {
    let s = serde_json::to_string(&SosState::Idle).unwrap();
    assert_eq!(s, r#""idle""#);

    let s = serde_json::to_string(&SosState::Cancelled).unwrap();
    assert_eq!(s, r#""cancelled""#);
}

#[test]
fn countdown_state_round_trips_json() {
    let original = SosState::Countdown { seconds_remaining: 7 };
    let json = serde_json::to_string(&original).unwrap();
    let restored: SosState = serde_json::from_str(&json).unwrap();
    assert_eq!(original, restored);
}

#[test]
fn active_state_round_trips_json() {
    let original = SosState::Active { event_id: "abc-123".to_string() };
    let json = serde_json::to_string(&original).unwrap();
    let restored: SosState = serde_json::from_str(&json).unwrap();
    assert_eq!(original, restored);
}
