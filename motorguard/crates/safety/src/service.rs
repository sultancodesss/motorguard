use motorguard_core::{
    models::{EmergencyContact, SosEvent},
    types::{SosEventId, SosTrigger, UserId},
    AppError, Result,
};
use motorguard_database::{EmergencyContactRepository, SosEventRepository};
use motorguard_notifications::{NotificationService, SosNotification};
use sqlx::SqlitePool;
use tracing::{info, warn};

/// Result of dispatching an SOS.
#[derive(Debug)]
pub struct SosDispatchResult {
    pub event: SosEvent,
    pub contacts_notified: usize,
}

pub struct SafetyService {
    pool: SqlitePool,
    notifier: NotificationService,
    /// If false, SMS and push alerts are suppressed (dev/test safety gate).
    real_sos_enabled: bool,
}

impl SafetyService {
    pub fn new(
        pool: SqlitePool,
        notifier: NotificationService,
        real_sos_enabled: bool,
    ) -> Self {
        Self {
            pool,
            notifier,
            real_sos_enabled,
        }
    }

    /// Dispatch an SOS event after the countdown completes.
    pub async fn dispatch_sos(
        &self,
        user_id: UserId,
        latitude: f64,
        longitude: f64,
        accuracy: f64,
        trigger: SosTrigger,
    ) -> Result<SosDispatchResult> {
        let sos_repo = SosEventRepository::new(&self.pool);
        let contact_repo = EmergencyContactRepository::new(&self.pool);

        // Create the SOS event record
        let event = SosEvent::new(user_id, latitude, longitude, accuracy, trigger);
        let created = sos_repo.create(&event).await?;

        // Fetch emergency contacts
        let contacts = contact_repo.list_by_user(user_id).await?;

        let mut notified = 0usize;
        for contact in &contacts {
            if self.real_sos_enabled {
                match self
                    .notifier
                    .send_sos_sms(contact, latitude, longitude)
                    .await
                {
                    Ok(_) => notified += 1,
                    Err(e) => warn!("Failed to notify {}: {}", contact.phone, e),
                }
            } else {
                // Dev mode — just log it
                info!(
                    "DEV SOS: Would notify {} ({}) at ({}, {})",
                    contact.name, contact.phone, latitude, longitude
                );
                notified += 1;
            }
        }

        sos_repo
            .update_contacts_notified(created.id, notified as i32)
            .await?;

        info!(
            "SOS dispatched for user {}: {} contacts notified",
            user_id, notified
        );

        Ok(SosDispatchResult {
            event: created,
            contacts_notified: notified,
        })
    }

    /// Resolve an active SOS event.
    pub async fn resolve_sos(&self, sos_id: SosEventId, reason: &str) -> Result<()> {
        let repo = SosEventRepository::new(&self.pool);
        let event = repo.find_by_id(sos_id).await?;

        use motorguard_core::types::SosStatus;
        if event.status == SosStatus::Resolved {
            return Err(AppError::SosAlreadyResolved);
        }

        repo.resolve(sos_id, reason).await?;
        info!("SOS {} resolved: {}", sos_id, reason);
        Ok(())
    }

    /// List emergency contacts for a user.
    pub async fn list_contacts(&self, user_id: UserId) -> Result<Vec<EmergencyContact>> {
        let repo = EmergencyContactRepository::new(&self.pool);
        repo.list_by_user(user_id).await
    }

    /// Add an emergency contact.
    pub async fn add_contact(
        &self,
        user_id: UserId,
        name: String,
        phone: String,
        relationship: Option<String>,
    ) -> Result<EmergencyContact> {
        let repo = EmergencyContactRepository::new(&self.pool);
        let contact = EmergencyContact::new(user_id, name, phone, relationship);
        repo.create(&contact).await
    }

    /// Remove an emergency contact.
    pub async fn remove_contact(
        &self,
        contact_id: motorguard_core::types::EmergencyContactId,
        user_id: UserId,
    ) -> Result<()> {
        let repo = EmergencyContactRepository::new(&self.pool);
        repo.delete(contact_id, user_id).await
    }
}
