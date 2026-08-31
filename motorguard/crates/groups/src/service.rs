use motorguard_core::{
    models::Group,
    types::{GroupId, GroupRole, UserId},
    AppError, Result,
};
use motorguard_database::GroupRepository;
use sqlx::SqlitePool;
use tracing::info;

pub struct GroupService {
    pool: SqlitePool,
}

impl GroupService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_group(
        &self,
        owner_id: UserId,
        name: String,
        description: Option<String>,
    ) -> Result<Group> {
        let repo = GroupRepository::new(&self.pool);
        let group = Group::new(name, description, owner_id);
        let created = repo.create(&group).await?;

        // Owner is always the first member
        repo.add_member(created.id, owner_id, GroupRole::Owner).await?;

        info!("Group '{}' created by user {}", created.name, owner_id);
        Ok(created)
    }

    pub async fn get_group(&self, group_id: GroupId) -> Result<Group> {
        let repo = GroupRepository::new(&self.pool);
        repo.find_by_id(group_id).await
    }

    pub async fn list_user_groups(&self, user_id: UserId) -> Result<Vec<Group>> {
        let repo = GroupRepository::new(&self.pool);
        repo.list_by_member(user_id).await
    }

    pub async fn join_group(
        &self,
        group_id: GroupId,
        user_id: UserId,
        invite_code: Option<String>,
    ) -> Result<()> {
        let repo = GroupRepository::new(&self.pool);
        let group = repo.find_by_id(group_id).await?;

        // Validate invite code if provided
        if let Some(code) = invite_code {
            if code != group.invite_code {
                return Err(AppError::Forbidden);
            }
        }

        repo.add_member(group_id, user_id, GroupRole::Member).await?;
        info!("User {} joined group {}", user_id, group_id);
        Ok(())
    }

    pub async fn leave_group(&self, group_id: GroupId, user_id: UserId) -> Result<()> {
        let repo = GroupRepository::new(&self.pool);
        let group = repo.find_by_id(group_id).await?;

        // Owners cannot leave — they must delete the group
        if group.owner_id == user_id {
            return Err(AppError::Validation(
                "Group owner cannot leave. Delete the group instead.".to_string(),
            ));
        }

        repo.remove_member(group_id, user_id).await?;
        info!("User {} left group {}", user_id, group_id);
        Ok(())
    }

    pub async fn get_member_count(&self, group_id: GroupId) -> Result<i64> {
        let repo = GroupRepository::new(&self.pool);
        repo.member_count(group_id).await
    }

    pub async fn assert_member(&self, group_id: GroupId, user_id: UserId) -> Result<()> {
        let repo = GroupRepository::new(&self.pool);
        if !repo.is_member(group_id, user_id).await? {
            Err(AppError::Forbidden)
        } else {
            Ok(())
        }
    }
}
