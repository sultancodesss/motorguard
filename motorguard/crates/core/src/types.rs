use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Newtype wrappers around `Uuid` to provide compile-time ID safety.
/// You can't accidentally pass a `GroupId` where a `UserId` is expected.

macro_rules! define_id {
    ($name:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type,
        )]
        #[sqlx(transparent)]
        pub struct $name(pub Uuid);

        impl $name {
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            pub fn from_uuid(id: Uuid) -> Self {
                Self(id)
            }

            pub fn as_uuid(&self) -> &Uuid {
                &self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::str::FromStr for $name {
            type Err = uuid::Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(s.parse::<Uuid>()?))
            }
        }
    };
}

define_id!(UserId);
define_id!(RideId);
define_id!(RidePointId);
define_id!(GroupId);
define_id!(MotorcycleId);
define_id!(SosEventId);
define_id!(EmergencyContactId);
define_id!(NotificationId);
define_id!(SessionId);

/// Ride lifecycle states.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
#[serde(rename_all = "snake_case")]
pub enum RideStatus {
    Pending,
    Active,
    Paused,
    Completed,
    Cancelled,
}

impl std::fmt::Display for RideStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RideStatus::Pending => "pending",
            RideStatus::Active => "active",
            RideStatus::Paused => "paused",
            RideStatus::Completed => "completed",
            RideStatus::Cancelled => "cancelled",
        };
        write!(f, "{s}")
    }
}

/// SOS event lifecycle states.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
#[serde(rename_all = "snake_case")]
pub enum SosStatus {
    Active,
    Resolved,
    FalseAlarm,
}

/// How an SOS was triggered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
#[serde(rename_all = "snake_case")]
pub enum SosTrigger {
    Manual,
    CrashDetection,
}

/// Group membership role.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
#[serde(rename_all = "snake_case")]
pub enum GroupRole {
    Owner,
    Admin,
    Member,
}

/// Notification category.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
#[serde(rename_all = "snake_case")]
pub enum NotificationKind {
    Sos,
    GroupInvite,
    RideComplete,
    SystemAlert,
}
