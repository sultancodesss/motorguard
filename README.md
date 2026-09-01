# 🏍️ MotorGuard

> **A smart motorcycle safety and group-riding platform built with Rust.**

MotorGuard is a motorcycle safety application designed to make riding **safer, smarter, and more connected**. It provides GPS-based ride tracking, real-time group riding, emergency SOS support, live maps, ride history, and secure authentication.

The backend is built entirely in **Rust** using **Axum**, **Tokio**, **SQLx**, and WebSockets.

---

## 🚀 Key Features

### 🗺️ Ride Tracking

* GPS-based route tracking
* Real-time speed monitoring
* Distance calculation
* Ride duration tracking
* Safety score
* Automatic ride recording

### 👥 Group Riding

* Create riding groups
* Join existing groups
* Real-time group member locations
* WebSocket-based live tracking
* Keep track of riding members

### 🆘 Emergency SOS

* Emergency SOS trigger
* Crash detection support
* Countdown before SOS activation
* Emergency contact notifications
* Real-alert protection through configuration

### 📍 Live Map

* Interactive map
* Real-time rider locations
* Group member markers
* Road/hazard reporting
* Route visualization

### 📊 Ride History

* Complete ride history
* Route information
* Distance and duration statistics
* Previous ride tracking

### 🔐 Authentication

* Phone number authentication
* OTP verification
* JWT-based sessions
* Password hashing with bcrypt
* Secure API authentication

---

## 🧠 How MotorGuard Works

```text
                    ┌──────────────────┐
                    │   MotorGuard     │
                    │     Client       │
                    └────────┬─────────┘
                             │
                             ▼
                    ┌──────────────────┐
                    │   Axum Server    │
                    │    REST + WS      │
                    └────────┬─────────┘
                             │
          ┌──────────────────┼──────────────────┐
          ▼                  ▼                  ▼
   ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
   │ Ride System │    │ Group Ride  │    │ Safety/SOS  │
   └──────┬──────┘    └──────┬──────┘    └──────┬──────┘
          │                  │                  │
          └──────────────────┼──────────────────┘
                             ▼
                    ┌──────────────────┐
                    │    Database      │
                    │ SQLite / Postgres│
                    └──────────────────┘
```

---

## 🛠️ Tech Stack

| Layer          | Technology           |
| -------------- | -------------------- |
| Language       | 🦀 Rust 2021         |
| Web Framework  | Axum 0.7             |
| Async Runtime  | Tokio                |
| Database       | SQLx                 |
| Development DB | SQLite               |
| Production DB  | PostgreSQL           |
| Real-Time      | WebSocket            |
| Authentication | JWT + bcrypt         |
| Serialization  | Serde                |
| Logging        | Tracing              |
| Frontend       | HTML5 + Tailwind CSS |

---

## 📁 Project Structure

```text
motorguard/
├── apps/
│   └── server/              # Axum HTTP server + WebSocket
│
├── crates/
│   ├── core/                # Shared models, errors and types
│   ├── auth/                # OTP, JWT and session logic
│   ├── rides/               # Ride recording and management
│   ├── location/            # GPS, distance and speed processing
│   ├── groups/              # Group management and live tracking
│   ├── safety/              # SOS and emergency contacts
│   ├── notifications/       # Notification system
│   ├── api/                 # API types and client
│   └── database/            # Database layer and migrations
│
├── apps/mobile/             # Frontend
├── assets/                  # Static assets
├── migrations/              # SQL migrations
├── docs/                    # Documentation
├── tests/                   # Integration tests
├── Cargo.toml
└── README.md
```

---

## ⚙️ Getting Started

### Prerequisites

Make sure you have:

* Rust 1.82+
* Cargo
* SQLite for development
* PostgreSQL for production

Install/update Rust:

```bash
rustup update stable
```

### 1. Clone the Repository

```bash
git clone https://github.com/sultancodesss/motorguard.git
cd motorguard
```

### 2. Configure Environment

```bash
cp .env.example .env
```

Open `.env` and configure your environment variables.

> ⚠️ Never commit your `.env` file or secret credentials to GitHub.

### 3. Run Database Migrations

```bash
cargo run -p motorguard-server -- migrate
```

### 4. Start the Server

```bash
cargo run -p motorguard-server
```

The application will be available at:

```text
http://localhost:8080
```

---

## 🧪 Run Tests

Run the complete workspace test suite:

```bash
cargo test --workspace
```

---

## 🔌 API Overview

| Method | Endpoint                | Description              |
| ------ | ----------------------- | ------------------------ |
| POST   | `/api/auth/request-otp` | Request phone OTP        |
| POST   | `/api/auth/verify-otp`  | Verify OTP and get token |
| GET    | `/api/users/me`         | Get current user         |
| PUT    | `/api/users/me`         | Update profile           |
| POST   | `/api/rides`            | Create a ride            |
| GET    | `/api/rides`            | Get ride history         |
| POST   | `/api/rides/:id/start`  | Start a ride             |
| POST   | `/api/rides/:id/finish` | Finish a ride            |
| GET    | `/api/groups`           | List groups              |
| POST   | `/api/groups`           | Create a group           |
| POST   | `/api/groups/:id/join`  | Join a group             |
| POST   | `/api/sos`              | Trigger SOS              |
| WS     | `/ws/groups/:id`        | Real-time group tracking |

For complete API documentation, see:

```text
docs/API.md
```

---

## 🔐 Security

MotorGuard follows several security practices:

* 🔑 Secrets are stored using environment variables
* 🎫 JWT authentication with expiry/refresh
* 🚦 Rate limiting for sensitive endpoints
* 🛡️ Input validation
* 🗄️ Parameterized SQL queries through SQLx
* 🆘 Real SOS alerts require explicit configuration

Real SOS functionality is protected behind:

```env
ENABLE_REAL_SOS=true
```

---

## 🎯 Project Goals

MotorGuard aims to solve common motorcycle safety and group-riding problems by providing:

* Better rider awareness
* Real-time group coordination
* Faster emergency response
* Ride analytics
* Location-based safety features
* A secure and scalable backend

---

## 🔮 Future Improvements

Potential future improvements include:

* 🤖 AI-based accident detection
* 📱 Native Android/iOS application
* 🧠 Advanced rider behavior analysis
* 🚨 Automatic emergency calling
* 🌦️ Weather-aware riding alerts
* 🛣️ Intelligent route safety scoring
* 📡 Offline GPS tracking
* 🔋 Battery-efficient background tracking
* 📊 Advanced rider analytics
* 🗺️ More detailed hazard detection

---

## 🤝 Contributing

Contributions are welcome!

### Steps

1. Fork the repository
2. Create a new branch

```bash
git checkout -b feature/your-feature
```

3. Make your changes
4. Run tests

```bash
cargo test --workspace
```

5. Commit your changes

```bash
git add .
git commit -m "feat: add your feature"
```

6. Push your branch

```bash
git push origin feature/your-feature
```

7. Open a Pull Request

---

## 📜 License

This project is licensed under the **MIT License**.

---

## 👨‍💻 Author

**Mohd Sultan**

GitHub: [@sultancodesss](https://github.com/sultancodesss)

---

## ⭐ Support

If you find **MotorGuard** useful, consider giving the repository a ⭐ on GitHub.

> **Ride Safe. Ride Smart. Ride Together. 🏍️🛡️**
