# MotorGuard — Architecture

## Overview

MotorGuard is a Rust monorepo (Cargo workspace) with a clear separation between application crates, library crates, and assets.

```
┌─────────────────────────────────────────────────────────┐
│                    Mobile Browser / App                  │
│        HTML + Tailwind CSS + Vanilla JS frontend         │
└────────────────────┬────────────────────────────────────┘
                     │ HTTP / WebSocket
┌────────────────────▼────────────────────────────────────┐
│                  apps/server (Axum)                      │
│   routes/ │ handlers/ │ middleware/ │ ws/ │ static/      │
└──┬────┬────┬──────┬────────────────────────────────────-─┘
   │    │    │      │
   ▼    ▼    ▼      ▼
┌─────┐ ┌──────┐ ┌───────┐ ┌──────────┐
│auth │ │rides │ │groups │ │ safety   │
└──┬──┘ └──┬───┘ └───┬───┘ └────┬─────┘
   │       │         │          │
   └───────┴─────────┴──────────┘
              │
        ┌─────▼──────┐
        │   core     │  ← shared models, errors, types
        └─────┬──────┘
              │
        ┌─────▼──────┐
        │  database  │  ← SQLx, migrations, queries
        └─────┬──────┘
              │
        ┌─────▼──────┐
        │  SQLite /  │
        │ PostgreSQL │
        └────────────┘
```

## Crates

### `crates/core`
Shared foundation used by every other crate.
- `models/` — domain types (User, Ride, Group, Location, SOSEvent…)
- `errors/` — typed `AppError` enum, `Result<T>` alias
- `types/` — newtype wrappers (UserId, RideId, GroupId…)
- No external dependencies beyond serde/chrono/uuid

### `crates/database`
All database concerns.
- SQLx pool setup (SQLite dev, PostgreSQL prod)
- Repository pattern — one module per domain entity
- Migration runner
- Query helpers

### `crates/auth`
- OTP generation (6-digit, expiry 10 min)
- OTP verification
- JWT token creation and validation
- Session management (access + refresh tokens)
- Password hashing (bcrypt) for email fallback

### `crates/rides`
- Ride lifecycle (create, start, pause, resume, finish)
- Ride point recording
- Safety score calculation
- Statistics aggregation

### `crates/location`
- `Location` struct (lat, lon, alt, speed, accuracy, timestamp)
- Distance calculation (Haversine formula)
- Speed calculation from successive points
- GPS accuracy validation
- Local offline buffer

### `crates/groups`
- Group CRUD
- Membership management
- WebSocket channel per group
- Real-time location broadcast message types

### `crates/safety`
- SOS state machine (idle → countdown → active → resolved)
- Emergency contact management
- SOS event creation and storage
- Notification dispatch (gated by ENABLE_REAL_SOS)

### `crates/notifications`
- Push notification abstraction (FCM / APNs)
- In-app notification model
- SMS dispatch abstraction

### `crates/api`
- Request/response DTOs
- API error response format
- Client types for frontend→backend communication

### `apps/server`
- Axum HTTP server entry point
- Route registration
- Handler implementations
- WebSocket upgrade and group channel management
- Static file serving (mobile frontend)
- Middleware: auth extraction, rate limiting, CORS, tracing

## Data Flow: Start Ride

```
1. User taps "Start Ride"
2. POST /api/rides        → creates ride record (status: pending)
3. POST /api/rides/:id/start → sets started_at, status: active
4. Mobile polls / streams GPS coordinates
5. POST /api/locations    → stores ride points (lat, lon, speed, timestamp)
   (offline: stored in IndexedDB, synced when back online)
6. User taps "Finish"
7. POST /api/rides/:id/finish
   → rides crate calculates: distance, duration, avg_speed, max_speed, safety_score
   → ride status: completed
8. GET /api/rides/:id     → returns full ride details for summary screen
```

## Data Flow: Group Real-Time

```
1. User joins group → POST /api/groups/:id/join
2. Mobile opens WebSocket → ws://host/ws/groups/:group_id?token=<jwt>
3. Server authenticates token, adds user to group channel
4. Mobile sends location every ~3 seconds:
   { "type": "location_update", "latitude": x, "longitude": y, "speed": z, "timestamp": t }
5. Server broadcasts to all other group members
6. Mobile receives updates and moves other riders' markers on map
7. On disconnect: user removed from active channel
```

## Data Flow: SOS

```
1. User triggers SOS (hold button or crash detection)
2. Frontend starts 10-second countdown
3. User can cancel within countdown
4. If not cancelled:
   a. GET current GPS location
   b. POST /api/sos → creates SOS event
   c. Server: fetches emergency contacts for user
   d. Server: sends SMS to each contact (if ENABLE_REAL_SOS=true)
   e. Server: sends push notification
   f. Returns SOS event ID
5. Frontend shows "SOS Active" screen with location
6. POST /api/sos/:id/resolve to close event
```

## Security Model

- **Authentication**: JWT Bearer tokens on all `/api/*` routes except `/api/auth/*`
- **Authorization**: Users can only access their own rides, groups they're members of
- **Rate Limiting**: OTP endpoint limited to 5 requests/hour per phone number
- **Input Validation**: All DTOs use `validator` crate
- **SQL Injection**: Prevented by SQLx parameterized queries
- **Secrets**: Loaded from environment, never compiled in
- **WebSocket Auth**: Token passed as query param on upgrade, validated before accepting

## Database Schema Summary

```
users              — id, phone, name, avatar_url, created_at
motorcycles        — id, user_id, make, model, year, plate
rides              — id, user_id, status, started_at, ended_at, distance, duration, …
ride_points        — id, ride_id, latitude, longitude, altitude, speed, accuracy, timestamp
groups             — id, name, description, owner_id, invite_code, created_at
group_members      — group_id, user_id, role, joined_at
locations          — id, user_id, latitude, longitude, speed, timestamp
emergency_contacts — id, user_id, name, phone, relationship
sos_events         — id, user_id, latitude, longitude, status, created_at, resolved_at
notifications      — id, user_id, title, body, read, created_at
sessions           — id, user_id, refresh_token_hash, expires_at, created_at
otp_codes          — id, phone, code_hash, expires_at, used
```
