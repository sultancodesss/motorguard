-- MotorGuard — Initial Schema Migration
-- Supports SQLite (dev) and PostgreSQL (prod)

-- ── Users ────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS users (
    id         TEXT      PRIMARY KEY,          -- UUID as text
    phone      TEXT      NOT NULL UNIQUE,
    name       TEXT,
    avatar_url TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_users_phone ON users(phone);

-- ── OTP codes ────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS otp_codes (
    id         INTEGER   PRIMARY KEY AUTOINCREMENT,
    phone      TEXT      NOT NULL,
    code_hash  TEXT      NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    used       BOOLEAN   NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_otp_phone ON otp_codes(phone);

-- ── Sessions ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS sessions (
    id                  TEXT      PRIMARY KEY,
    user_id             TEXT      NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    refresh_token_hash  TEXT      NOT NULL,
    expires_at          TIMESTAMP NOT NULL,
    created_at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id);

-- ── Motorcycles ───────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS motorcycles (
    id         TEXT      PRIMARY KEY,
    user_id    TEXT      NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    make       TEXT      NOT NULL,
    model      TEXT      NOT NULL,
    year       INTEGER   NOT NULL,
    plate      TEXT,
    color      TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_motorcycles_user_id ON motorcycles(user_id);

-- ── Rides ─────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS rides (
    id                 TEXT      PRIMARY KEY,
    user_id            TEXT      NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name               TEXT,
    status             TEXT      NOT NULL DEFAULT 'pending',  -- pending|active|paused|completed|cancelled
    started_at         TIMESTAMP,
    ended_at           TIMESTAMP,
    distance_miles     REAL      NOT NULL DEFAULT 0.0,
    duration_seconds   INTEGER   NOT NULL DEFAULT 0,
    average_speed_mph  REAL      NOT NULL DEFAULT 0.0,
    max_speed_mph      REAL      NOT NULL DEFAULT 0.0,
    safety_score       REAL,
    route_summary      TEXT,
    created_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_rides_user_id         ON rides(user_id);
CREATE INDEX IF NOT EXISTS idx_rides_status          ON rides(status);
CREATE INDEX IF NOT EXISTS idx_rides_user_id_status  ON rides(user_id, status);

-- ── Ride Points ───────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS ride_points (
    id          TEXT      PRIMARY KEY,
    ride_id     TEXT      NOT NULL REFERENCES rides(id) ON DELETE CASCADE,
    latitude    REAL      NOT NULL,
    longitude   REAL      NOT NULL,
    altitude    REAL,
    speed       REAL      NOT NULL DEFAULT 0.0,
    accuracy    REAL      NOT NULL DEFAULT 0.0,
    recorded_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ride_points_ride_id     ON ride_points(ride_id);
CREATE INDEX IF NOT EXISTS idx_ride_points_recorded_at ON ride_points(recorded_at);

-- ── Groups ────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS groups (
    id          TEXT      PRIMARY KEY,
    name        TEXT      NOT NULL,
    description TEXT,
    owner_id    TEXT      NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    invite_code TEXT      NOT NULL UNIQUE,
    is_active   BOOLEAN   NOT NULL DEFAULT 1,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_groups_owner_id    ON groups(owner_id);
CREATE INDEX IF NOT EXISTS idx_groups_invite_code ON groups(invite_code);

-- ── Group Members ─────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS group_members (
    group_id  TEXT      NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    user_id   TEXT      NOT NULL REFERENCES users(id)  ON DELETE CASCADE,
    role      TEXT      NOT NULL DEFAULT 'member',     -- owner|admin|member
    joined_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (group_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_group_members_user_id  ON group_members(user_id);
CREATE INDEX IF NOT EXISTS idx_group_members_group_id ON group_members(group_id);

-- ── Locations (latest per user, for group tracking) ───────────────────────────
CREATE TABLE IF NOT EXISTS locations (
    user_id     TEXT      PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    latitude    REAL      NOT NULL,
    longitude   REAL      NOT NULL,
    speed       REAL      NOT NULL DEFAULT 0.0,
    accuracy    REAL      NOT NULL DEFAULT 0.0,
    recorded_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ── Emergency Contacts ────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS emergency_contacts (
    id           TEXT      PRIMARY KEY,
    user_id      TEXT      NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name         TEXT      NOT NULL,
    phone        TEXT      NOT NULL,
    relationship TEXT,
    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_emergency_contacts_user_id ON emergency_contacts(user_id);

-- ── SOS Events ────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS sos_events (
    id                 TEXT      PRIMARY KEY,
    user_id            TEXT      NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    latitude           REAL      NOT NULL,
    longitude          REAL      NOT NULL,
    accuracy           REAL      NOT NULL DEFAULT 0.0,
    trigger            TEXT      NOT NULL DEFAULT 'manual',  -- manual|crash_detection
    status             TEXT      NOT NULL DEFAULT 'active',  -- active|resolved|false_alarm
    contacts_notified  INTEGER   NOT NULL DEFAULT 0,
    created_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    resolved_at        TIMESTAMP,
    resolve_reason     TEXT
);

CREATE INDEX IF NOT EXISTS idx_sos_events_user_id ON sos_events(user_id);
CREATE INDEX IF NOT EXISTS idx_sos_events_status  ON sos_events(status);

-- ── Notifications ─────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS notifications (
    id         TEXT      PRIMARY KEY,
    user_id    TEXT      NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    kind       TEXT      NOT NULL DEFAULT 'system_alert',  -- sos|group_invite|ride_complete|system_alert
    title      TEXT      NOT NULL,
    body       TEXT      NOT NULL,
    is_read    BOOLEAN   NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_notifications_user_id ON notifications(user_id);
CREATE INDEX IF NOT EXISTS idx_notifications_is_read ON notifications(is_read);
