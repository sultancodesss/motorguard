# MotorGuard

A production-ready motorcycle safety and group riding application built entirely in Rust.

## Features

- **Ride Tracking** — GPS-based route recording, speed, distance, duration, and safety score
- **Group Riding** — Real-time group location via WebSocket, create/join groups
- **SOS System** — Crash detection with countdown, emergency contact notifications
- **Live Map** — Interactive map with hazard reporting and group member markers
- **Ride History** — Full ride log with route maps and statistics
- **Authentication** — Phone-based OTP login with JWT session management

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust (edition 2021) |
| Web Framework | Axum 0.7 |
| Async Runtime | Tokio |
| Database | SQLx + SQLite (dev) / PostgreSQL (prod) |
| Real-Time | WebSocket (tokio-tungstenite) |
| Auth | JWT + bcrypt |
| Serialization | Serde |
| Logging | Tracing |
| Frontend | HTML5 + Tailwind CSS (served by Axum) |

## Project Structure

```
motorguard/
├── apps/
│   └── server/          # Axum HTTP server + WebSocket
├── crates/
│   ├── core/            # Shared models, errors, types
│   ├── auth/            # OTP, JWT, session logic
│   ├── rides/           # Ride recording and management
│   ├── location/        # GPS processing, distance, speed
│   ├── groups/          # Group management, real-time tracking
│   ├── safety/          # SOS, emergency contacts
│   ├── notifications/   # Push notifications
│   ├── api/             # API types and client
│   └── database/        # Database layer, migrations
├── apps/mobile/         # HTML/JS frontend
├── assets/              # Static assets
├── migrations/          # SQL migrations
├── docs/                # Documentation
└── tests/               # Integration tests
```

## Getting Started

### Prerequisites

- Rust 1.82+ (`rustup update stable`)
- SQLite (for development) or PostgreSQL (for production)

### Setup

```bash
# Clone and enter the project
cd motorguard

# Copy environment config
cp .env.example .env
# Edit .env with your values

# Run database migrations
cargo run -p motorguard-server -- migrate

# Start the development server
cargo run -p motorguard-server
```

The server starts on `http://localhost:8080`.
Open `http://localhost:8080` in a browser to see the app.

### Running Tests

```bash
cargo test --workspace
```

## API Overview

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/auth/request-otp` | Request phone OTP |
| POST | `/api/auth/verify-otp` | Verify OTP + get token |
| GET | `/api/users/me` | Get current user |
| PUT | `/api/users/me` | Update profile |
| POST | `/api/rides` | Create new ride |
| GET | `/api/rides` | List ride history |
| POST | `/api/rides/:id/start` | Start a ride |
| POST | `/api/rides/:id/finish` | Finish a ride |
| GET | `/api/groups` | List groups |
| POST | `/api/groups` | Create group |
| POST | `/api/groups/:id/join` | Join group |
| POST | `/api/sos` | Trigger SOS |
| WS | `/ws/groups/:id` | Real-time group tracking |

See `docs/API.md` for full documentation.

## Security

- All secrets via environment variables — never hard-coded
- JWT tokens with expiry and refresh
- Rate limiting on sensitive endpoints
- SOS real-alerts gated by `ENABLE_REAL_SOS=true`
- Input validation on all API endpoints
- SQL parameterized queries via SQLx

## License

MIT
