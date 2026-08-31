/// Tests for JWT creation and verification.

use motorguard_auth::JwtService;
use motorguard_core::types::UserId;

fn make_jwt() -> JwtService {
    JwtService::new("test-secret-key-for-unit-tests", 24, 30)
}

// ── Access tokens ─────────────────────────────────────────────────────────────

#[test]
fn access_token_roundtrip() {
    let svc     = make_jwt();
    let user_id = UserId::new();
    let token   = svc.create_access_token(user_id).unwrap();
    let claims  = svc.verify_access_token(&token).unwrap();
    assert_eq!(claims.user_id().unwrap(), user_id);
}

#[test]
fn access_token_type_is_access() {
    let svc   = make_jwt();
    let token = svc.create_access_token(UserId::new()).unwrap();
    let claims = svc.verify(&token).unwrap();
    assert_eq!(claims.typ, "access");
}

#[test]
fn refresh_token_type_is_refresh() {
    let svc   = make_jwt();
    let token = svc.create_refresh_token(UserId::new()).unwrap();
    let claims = svc.verify(&token).unwrap();
    assert_eq!(claims.typ, "refresh");
}

#[test]
fn verify_access_rejects_refresh_token() {
    let svc   = make_jwt();
    let token = svc.create_refresh_token(UserId::new()).unwrap();
    let result = svc.verify_access_token(&token);
    assert!(result.is_err(), "verify_access_token should reject a refresh token");
}

#[test]
fn tampered_token_rejected() {
    let svc   = make_jwt();
    let token = svc.create_access_token(UserId::new()).unwrap();
    let bad   = token + "x";                   // append a char to corrupt signature
    let result = svc.verify_access_token(&bad);
    assert!(result.is_err(), "Tampered token must be rejected");
}

#[test]
fn wrong_secret_rejected() {
    let svc1  = JwtService::new("secret-one", 24, 30);
    let svc2  = JwtService::new("secret-two", 24, 30);
    let token = svc1.create_access_token(UserId::new()).unwrap();
    let result = svc2.verify_access_token(&token);
    assert!(result.is_err(), "Token signed with different secret must fail");
}

#[test]
fn expired_token_rejected() {
    // expiry = 0 hours → already expired at creation
    let svc   = JwtService::new("test-secret", 0, 0);
    let token = svc.create_access_token(UserId::new()).unwrap();
    // Small sleep not needed — 0h expiry means exp == iat, which is in the past
    let result = svc.verify_access_token(&token);
    assert!(result.is_err(), "Zero-hour expiry token must be rejected");
}

#[test]
fn garbage_token_rejected() {
    let svc    = make_jwt();
    let result = svc.verify_access_token("not.a.jwt");
    assert!(result.is_err());
}

#[test]
fn empty_token_rejected() {
    let svc    = make_jwt();
    let result = svc.verify_access_token("");
    assert!(result.is_err());
}

// ── Claims helpers ────────────────────────────────────────────────────────────

#[test]
fn claims_user_id_parses_correctly() {
    let svc     = make_jwt();
    let user_id = UserId::new();
    let token   = svc.create_access_token(user_id).unwrap();
    let claims  = svc.verify(&token).unwrap();
    let parsed  = claims.user_id().unwrap();
    assert_eq!(parsed, user_id);
}

#[test]
fn different_users_get_different_tokens() {
    let svc = make_jwt();
    let t1  = svc.create_access_token(UserId::new()).unwrap();
    let t2  = svc.create_access_token(UserId::new()).unwrap();
    assert_ne!(t1, t2, "Each user should get a unique token");
}
