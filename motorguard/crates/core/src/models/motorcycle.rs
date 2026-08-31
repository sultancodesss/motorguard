use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::types::{MotorcycleId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Motorcycle {
    pub id: MotorcycleId,
    pub user_id: UserId,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub plate: Option<String>,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Motorcycle {
    pub fn new(user_id: UserId, make: String, model: String, year: i32) -> Self {
        let now = Utc::now();
        Self {
            id: MotorcycleId::new(),
            user_id,
            make,
            model,
            year,
            plate: None,
            color: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn display_name(&self) -> String {
        format!("{} {} {}", self.year, self.make, self.model)
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::UserId;

    #[test]
    fn display_name_format() {
        let uid = UserId::new();
        let m = Motorcycle::new(uid, "Honda".to_string(), "CBR600RR".to_string(), 2022);
        assert_eq!(m.display_name(), "2022 Honda CBR600RR");
    }

    #[test]
    fn new_has_no_plate() {
        let uid = UserId::new();
        let m = Motorcycle::new(uid, "Kawasaki".to_string(), "Ninja".to_string(), 2021);
        assert!(m.plate.is_none());
        assert!(m.color.is_none());
    }
}
