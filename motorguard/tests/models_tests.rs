/// Tests for core model constructors and helpers.

use motorguard_core::{
    models::{Ride, User, Group, EmergencyContact},
    types::{RideStatus, UserId},
};

// ── User ──────────────────────────────────────────────────────────────────────

#[test]
fn user_new_has_no_name() {
    let u = User::new("+15551234567".to_string());
    assert_eq!(u.phone, "+15551234567");
    assert!(u.name.is_none());
    assert!(u.avatar_url.is_none());
}

#[test]
fn user_display_name_falls_back_to_rider() {
    let u = User::new("+1555".to_string());
    assert_eq!(u.display_name(), "Rider");
}

#[test]
fn user_display_name_uses_name_when_set() {
    let mut u = User::new("+1555".to_string());
    u.name = Some("Alex".to_string());
    assert_eq!(u.display_name(), "Alex");
}

#[test]
fn user_ids_are_unique() {
    let u1 = User::new("+1111".to_string());
    let u2 = User::new("+2222".to_string());
    assert_ne!(u1.id, u2.id);
}

// ── Ride ──────────────────────────────────────────────────────────────────────

#[test]
fn ride_new_starts_pending() {
    let uid = UserId::new();
    let r   = Ride::new(uid, Some("Test Ride".to_string()));
    assert_eq!(r.status, RideStatus::Pending);
    assert!(r.started_at.is_none());
    assert!(r.ended_at.is_none());
    assert_eq!(r.distance_miles, 0.0);
}

#[test]
fn ride_duration_display_minutes() {
    let uid = UserId::new();
    let mut r = Ride::new(uid, None);
    r.duration_seconds = 1680; // 28 minutes
    assert_eq!(r.duration_display(), "28 min");
}

#[test]
fn ride_duration_display_hours_and_minutes() {
    let uid = UserId::new();
    let mut r = Ride::new(uid, None);
    r.duration_seconds = 6300; // 1h 45m
    assert_eq!(r.duration_display(), "1h 45m");
}

#[test]
fn ride_duration_display_zero() {
    let uid = UserId::new();
    let r = Ride::new(uid, None);
    assert_eq!(r.duration_display(), "0 min");
}

#[test]
fn ride_ids_are_unique() {
    let uid = UserId::new();
    let r1 = Ride::new(uid, None);
    let r2 = Ride::new(uid, None);
    assert_ne!(r1.id, r2.id);
}

// ── Group ─────────────────────────────────────────────────────────────────────

#[test]
fn group_new_has_invite_code() {
    let uid = UserId::new();
    let g   = Group::new("Test Group".to_string(), None, uid);
    assert!(!g.invite_code.is_empty(), "Invite code should be generated");
    assert_eq!(g.invite_code.len(), 6, "Invite code should be 6 chars");
}

#[test]
fn group_is_active_on_creation() {
    let uid = UserId::new();
    let g   = Group::new("Riders".to_string(), None, uid);
    assert!(g.is_active);
}

#[test]
fn group_owner_is_set() {
    let uid = UserId::new();
    let g   = Group::new("Group".to_string(), None, uid);
    assert_eq!(g.owner_id, uid);
}

// ── Emergency Contact ─────────────────────────────────────────────────────────

#[test]
fn emergency_contact_new() {
    let uid = UserId::new();
    let c   = EmergencyContact::new(
        uid,
        "Jane".to_string(),
        "+15559876543".to_string(),
        Some("spouse".to_string()),
    );
    assert_eq!(c.name,         "Jane");
    assert_eq!(c.phone,        "+15559876543");
    assert_eq!(c.relationship, Some("spouse".to_string()));
    assert_eq!(c.user_id,      uid);
}

// ── Type system ───────────────────────────────────────────────────────────────

#[test]
fn user_id_round_trips_string() {
    let id = UserId::new();
    let s  = id.to_string();
    let parsed: UserId = s.parse().unwrap();
    assert_eq!(id, parsed);
}

#[test]
fn ride_status_display() {
    assert_eq!(RideStatus::Active.to_string(),    "active");
    assert_eq!(RideStatus::Completed.to_string(), "completed");
    assert_eq!(RideStatus::Pending.to_string(),   "pending");
    assert_eq!(RideStatus::Paused.to_string(),    "paused");
    assert_eq!(RideStatus::Cancelled.to_string(), "cancelled");
}
