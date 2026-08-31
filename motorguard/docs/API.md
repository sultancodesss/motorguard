# MotorGuard — API Reference

Base URL: `http://localhost:8080`

All protected endpoints require:
```
Authorization: Bearer <access_token>
```

---

## Authentication

### POST /api/auth/request-otp
Request a 6-digit OTP sent via SMS.

**Request:**
```json
{ "phone": "+15551234567" }
```

**Response 200:**
```json
{ "message": "OTP sent", "expires_in_seconds": 600 }
```

**Rate limit:** 5 per hour per phone number.

---

### POST /api/auth/verify-otp
Verify OTP and receive access + refresh tokens.

**Request:**
```json
{ "phone": "+15551234567", "code": "123456" }
```

**Response 200:**
```json
{
  "access_token": "eyJ...",
  "refresh_token": "eyJ...",
  "token_type": "Bearer",
  "expires_in": 86400,
  "user": {
    "id": "uuid",
    "phone": "+15551234567",
    "name": null,
    "avatar_url": null,
    "is_new_user": true
  }
}
```

---

### POST /api/auth/refresh
Exchange refresh token for new access token.

**Request:**
```json
{ "refresh_token": "eyJ..." }
```

**Response 200:**
```json
{ "access_token": "eyJ...", "expires_in": 86400 }
```

---

### POST /api/auth/logout
Invalidate current session.

**Response 204:** No content.

---

## Users

### GET /api/users/me
Get current user profile.

**Response 200:**
```json
{
  "id": "uuid",
  "phone": "+15551234567",
  "name": "Alex",
  "avatar_url": "https://...",
  "created_at": "2024-01-15T08:00:00Z",
  "stats": {
    "total_rides": 124,
    "total_miles": 2400.5,
    "safety_score": 98
  }
}
```

---

### PUT /api/users/me
Update profile.

**Request:**
```json
{ "name": "Alex", "avatar_url": "https://..." }
```

---

### GET /api/users/me/motorcycle
Get user's motorcycle details.

### PUT /api/users/me/motorcycle
Update motorcycle.

**Request:**
```json
{ "make": "Honda", "model": "CBR600RR", "year": 2022, "plate": "7ABC123" }
```

---

## Rides

### POST /api/rides
Create a new ride session.

**Request:**
```json
{ "name": "Morning Commute" }
```

**Response 201:**
```json
{
  "id": "uuid",
  "user_id": "uuid",
  "name": "Morning Commute",
  "status": "pending",
  "created_at": "2024-01-15T08:00:00Z"
}
```

---

### GET /api/rides
List ride history (paginated).

**Query params:** `page=1&per_page=20&filter=all|commute|weekend`

**Response 200:**
```json
{
  "rides": [
    {
      "id": "uuid",
      "name": "Morning Commute",
      "status": "completed",
      "started_at": "2024-10-24T08:15:00Z",
      "ended_at": "2024-10-24T08:43:00Z",
      "distance_miles": 12.4,
      "duration_minutes": 28,
      "average_speed_mph": 26.6,
      "max_speed_mph": 55.0,
      "safety_score": 98,
      "route_summary": "Downtown Office via I-95"
    }
  ],
  "total": 124,
  "page": 1,
  "per_page": 20
}
```

---

### GET /api/rides/:id
Get ride details including route points.

---

### POST /api/rides/:id/start
Start a ride.

**Response 200:**
```json
{ "id": "uuid", "status": "active", "started_at": "2024-01-15T08:00:00Z" }
```

---

### POST /api/rides/:id/pause
Pause an active ride.

### POST /api/rides/:id/resume
Resume a paused ride.

---

### POST /api/rides/:id/finish
Finish a ride.

**Response 200:**
```json
{
  "id": "uuid",
  "status": "completed",
  "distance_miles": 12.4,
  "duration_minutes": 28,
  "average_speed_mph": 26.6,
  "max_speed_mph": 55.0,
  "safety_score": 98,
  "ended_at": "2024-01-15T08:28:00Z"
}
```

---

### POST /api/rides/:id/points
Submit GPS points for active ride.

**Request:**
```json
{
  "points": [
    {
      "latitude": 37.7749,
      "longitude": -122.4194,
      "altitude": 52.3,
      "speed": 35.5,
      "accuracy": 8.0,
      "timestamp": "2024-01-15T08:05:00Z"
    }
  ]
}
```

---

## Groups

### GET /api/groups
List user's groups.

**Response 200:**
```json
{
  "groups": [
    {
      "id": "uuid",
      "name": "Bay Area Sportbikers",
      "description": "...",
      "member_count": 1240,
      "active_count": 12,
      "is_member": true,
      "created_at": "2023-01-01T00:00:00Z"
    }
  ]
}
```

---

### POST /api/groups
Create a group.

**Request:**
```json
{
  "name": "PCH Sunset Run",
  "description": "Sunday evening coastal ride"
}
```

---

### GET /api/groups/:id
Get group details with members.

---

### POST /api/groups/:id/join
Join a group (public) or join by invite code.

**Request:**
```json
{ "invite_code": "ABC123" }
```

---

### POST /api/groups/:id/leave
Leave a group.

---

### DELETE /api/groups/:id
Delete a group (owner only).

---

## Locations

### POST /api/locations
Submit current location (for group sharing).

**Request:**
```json
{
  "latitude": 37.7749,
  "longitude": -122.4194,
  "speed": 35.5,
  "accuracy": 8.0,
  "timestamp": "2024-01-15T08:05:00Z"
}
```

---

## SOS

### POST /api/sos
Trigger an SOS event.

**Request:**
```json
{
  "latitude": 37.7749,
  "longitude": -122.4194,
  "accuracy": 10.0,
  "trigger": "manual|crash_detection"
}
```

**Response 201:**
```json
{
  "id": "uuid",
  "status": "active",
  "latitude": 37.7749,
  "longitude": -122.4194,
  "created_at": "2024-01-15T08:00:00Z",
  "contacts_notified": 2,
  "message": "SOS dispatched. Contacts notified."
}
```

---

### POST /api/sos/:id/resolve
Mark SOS event as resolved.

**Request:**
```json
{ "reason": "false_alarm|resolved|assisted" }
```

---

### GET /api/emergency-contacts
List emergency contacts.

### POST /api/emergency-contacts
Add an emergency contact.

**Request:**
```json
{ "name": "Jane Doe", "phone": "+15559876543", "relationship": "spouse" }
```

### DELETE /api/emergency-contacts/:id
Remove an emergency contact.

---

## WebSocket: Group Real-Time

### WS /ws/groups/:group_id

**Connect:** Include JWT as query param:
```
ws://localhost:8080/ws/groups/uuid?token=eyJ...
```

**Client → Server messages:**

```json
{
  "type": "location_update",
  "latitude": 37.7749,
  "longitude": -122.4194,
  "speed": 35.5,
  "heading": 270.0,
  "timestamp": "2024-01-15T08:05:00Z"
}
```

```json
{ "type": "ping" }
```

**Server → Client messages:**

```json
{
  "type": "member_location",
  "user_id": "uuid",
  "name": "Alex",
  "latitude": 37.7749,
  "longitude": -122.4194,
  "speed": 35.5,
  "heading": 270.0,
  "timestamp": "2024-01-15T08:05:00Z"
}
```

```json
{
  "type": "member_joined",
  "user_id": "uuid",
  "name": "Alex"
}
```

```json
{
  "type": "member_left",
  "user_id": "uuid"
}
```

```json
{ "type": "pong" }
```

---

## Error Responses

All errors follow this format:

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Phone number is invalid",
    "details": { "field": "phone", "reason": "must be E.164 format" }
  }
}
```

### Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `UNAUTHORIZED` | 401 | Missing or invalid token |
| `FORBIDDEN` | 403 | Insufficient permissions |
| `NOT_FOUND` | 404 | Resource not found |
| `VALIDATION_ERROR` | 422 | Invalid request data |
| `RATE_LIMITED` | 429 | Too many requests |
| `INTERNAL_ERROR` | 500 | Server error |
| `OTP_EXPIRED` | 422 | OTP code has expired |
| `OTP_INVALID` | 422 | Wrong OTP code |
| `RIDE_NOT_ACTIVE` | 422 | Operation requires active ride |
| `ALREADY_MEMBER` | 409 | Already a group member |
