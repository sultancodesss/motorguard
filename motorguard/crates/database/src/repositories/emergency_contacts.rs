use motorguard_core::{
    models::EmergencyContact,
    types::{EmergencyContactId, UserId},
    AppError, Result,
};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct EmergencyContactRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> EmergencyContactRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn list_by_user(&self, user_id: UserId) -> Result<Vec<EmergencyContact>> {
        let user_id_str = user_id.to_string();
        let rows = sqlx::query!(
            r#"SELECT id, user_id, name, phone, relationship, created_at FROM emergency_contacts WHERE user_id = ? ORDER BY created_at ASC"#,
            user_id_str
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|row| EmergencyContact {
                id: EmergencyContactId::from_uuid(Uuid::parse_str(&row.id).unwrap_or_default()),
                user_id: UserId::from_uuid(Uuid::parse_str(&row.user_id).unwrap_or_default()),
                name: row.name,
                phone: row.phone,
                relationship: row.relationship,
                created_at: row.created_at.and_utc(),
            })
            .collect())
    }

    pub async fn create(&self, contact: &EmergencyContact) -> Result<EmergencyContact> {
        let id_str = contact.id.to_string();
        let user_id_str = contact.user_id.to_string();
        sqlx::query!(
            r#"INSERT INTO emergency_contacts (id, user_id, name, phone, relationship, created_at) VALUES (?, ?, ?, ?, ?, ?)"#,
            id_str,
            user_id_str,
            contact.name,
            contact.phone,
            contact.relationship,
            contact.created_at,
        )
        .execute(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(contact.clone())
    }

    pub async fn delete(&self, id: EmergencyContactId, user_id: UserId) -> Result<()> {
        let id_str = id.to_string();
        let user_id_str = user_id.to_string();
        let result = sqlx::query!(
            r#"DELETE FROM emergency_contacts WHERE id = ? AND user_id = ?"#,
            id_str,
            user_id_str
        )
        .execute(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Emergency contact".to_string()));
        }
        Ok(())
    }
}
