use motorguard_core::models::EmergencyContact;
use tracing::{info, warn};

/// Payload describing an SOS alert notification.
#[derive(Debug, Clone)]
pub struct SosNotification {
    pub recipient_name: String,
    pub recipient_phone: String,
    pub rider_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub maps_url: String,
}

impl SosNotification {
    pub fn new(contact: &EmergencyContact, rider_name: &str, lat: f64, lon: f64) -> Self {
        let maps_url = format!(
            "https://www.google.com/maps?q={},{}",
            lat, lon
        );
        Self {
            recipient_name: contact.name.clone(),
            recipient_phone: contact.phone.clone(),
            rider_name: rider_name.to_string(),
            latitude: lat,
            longitude: lon,
            maps_url,
        }
    }

    pub fn sms_body(&self) -> String {
        format!(
            "EMERGENCY: {} may need help! Last known location: {} - Open map: {}",
            self.rider_name, self.format_coords(), self.maps_url
        )
    }

    fn format_coords(&self) -> String {
        format!("{:.5}, {:.5}", self.latitude, self.longitude)
    }
}

/// Abstraction over SMS and push notification providers.
///
/// In dev mode (or when provider credentials are absent) all sends are
/// logged rather than dispatched to avoid accidental real alerts.
#[derive(Clone)]
pub struct NotificationService {
    twilio_sid: Option<String>,
    twilio_token: Option<String>,
    twilio_from: Option<String>,
    dev_mode: bool,
}

impl NotificationService {
    pub fn new(
        twilio_sid: Option<String>,
        twilio_token: Option<String>,
        twilio_from: Option<String>,
        dev_mode: bool,
    ) -> Self {
        Self {
            twilio_sid,
            twilio_token,
            twilio_from,
            dev_mode,
        }
    }

    /// Send an SOS SMS to an emergency contact.
    pub async fn send_sos_sms(
        &self,
        contact: &EmergencyContact,
        latitude: f64,
        longitude: f64,
    ) -> anyhow::Result<()> {
        let notif = SosNotification::new(contact, "Rider", latitude, longitude);

        if self.dev_mode || self.twilio_sid.is_none() {
            info!(
                "DEV: Would SMS {} at {}: {}",
                notif.recipient_name,
                notif.recipient_phone,
                notif.sms_body()
            );
            return Ok(());
        }

        let sid = self.twilio_sid.as_deref().unwrap();
        let token = self.twilio_token.as_deref().unwrap_or("");
        let from = self.twilio_from.as_deref().unwrap_or("");

        let url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            sid
        );

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .basic_auth(sid, Some(token))
            .form(&[
                ("To", notif.recipient_phone.as_str()),
                ("From", from),
                ("Body", notif.sms_body().as_str()),
            ])
            .send()
            .await?;

        if resp.status().is_success() {
            info!("SOS SMS sent to {}", notif.recipient_phone);
        } else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            warn!("Twilio error {}: {}", status, body);
            anyhow::bail!("Twilio returned {}", status);
        }

        Ok(())
    }
}
