use motorguard_core::{
    models::{Group, GroupMember},
    types::{GroupId, UserId, GroupRole},
    AppError, Result,
};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

pub struct GroupRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> GroupRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, group: &Group) -> Result<Group> {
        let id_str = group.id.to_string();
        let owner_id_str = group.owner_id.to_string();
        sqlx::query!(
            r#"
            INSERT INTO groups (id, name, description, owner_id, invite_code, is_active, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            id_str,
            group.name,
            group.description,
            owner_id_str,
            group.invite_code,
            group.is_active,
            group.created_at,
            group.updated_at,
        )
        .execute(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(group.clone())
    }

    pub async fn find_by_id(&self, id: GroupId) -> Result<Group> {
        let id_str = id.to_string();
        let row = sqlx::query!(
            r#"SELECT id, name, description, owner_id, invite_code, is_active, created_at, updated_at FROM groups WHERE id = ?"#,
            id_str
        )
        .fetch_one(self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Group".to_string()),
            _ => AppError::Database(e.to_string()),
        })?;

        Ok(Group {
            id: GroupId::from_uuid(Uuid::parse_str(&row.id).unwrap_or_default()),
            name: row.name,
            description: row.description,
            owner_id: UserId::from_uuid(Uuid::parse_str(&row.owner_id).unwrap_or_default()),
            invite_code: row.invite_code,
            is_active: row.is_active,
            created_at: row.created_at.and_utc(),
            updated_at: row.updated_at.and_utc(),
        })
    }

    pub async fn list_by_member(&self, user_id: UserId) -> Result<Vec<Group>> {
        let user_id_str = user_id.to_string();
        let rows = sqlx::query!(
            r#"
            SELECT g.id, g.name, g.description, g.owner_id, g.invite_code, g.is_active, g.created_at, g.updated_at
            FROM groups g
            INNER JOIN group_members gm ON g.id = gm.group_id
            WHERE gm.user_id = ?
            ORDER BY g.created_at DESC
            "#,
            user_id_str
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|row| Group {
                id: GroupId::from_uuid(Uuid::parse_str(&row.id).unwrap_or_default()),
                name: row.name,
                description: row.description,
                owner_id: UserId::from_uuid(Uuid::parse_str(&row.owner_id).unwrap_or_default()),
                invite_code: row.invite_code,
                is_active: row.is_active,
                created_at: row.created_at.and_utc(),
                updated_at: row.updated_at.and_utc(),
            })
            .collect())
    }

    pub async fn is_member(&self, group_id: GroupId, user_id: UserId) -> Result<bool> {
        let group_id_str = group_id.to_string();
        let user_id_str = user_id.to_string();
        let row = sqlx::query!(
            r#"SELECT COUNT(*) as count FROM group_members WHERE group_id = ? AND user_id = ?"#,
            group_id_str,
            user_id_str
        )
        .fetch_one(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(row.count > 0)
    }

    pub async fn add_member(&self, group_id: GroupId, user_id: UserId, role: GroupRole) -> Result<()> {
        if self.is_member(group_id, user_id).await? {
            return Err(AppError::AlreadyMember);
        }
        let group_id_str = group_id.to_string();
        let user_id_str = user_id.to_string();
        let role_str = format!("{:?}", role).to_lowercase();
        let now = Utc::now();
        sqlx::query!(
            r#"INSERT INTO group_members (group_id, user_id, role, joined_at) VALUES (?, ?, ?, ?)"#,
            group_id_str,
            user_id_str,
            role_str,
            now,
        )
        .execute(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn remove_member(&self, group_id: GroupId, user_id: UserId) -> Result<()> {
        let group_id_str = group_id.to_string();
        let user_id_str = user_id.to_string();
        sqlx::query!(
            r#"DELETE FROM group_members WHERE group_id = ? AND user_id = ?"#,
            group_id_str,
            user_id_str
        )
        .execute(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn member_count(&self, group_id: GroupId) -> Result<i64> {
        let group_id_str = group_id.to_string();
        let row = sqlx::query!(
            r#"SELECT COUNT(*) as count FROM group_members WHERE group_id = ?"#,
            group_id_str
        )
        .fetch_one(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(row.count)
    }
}
