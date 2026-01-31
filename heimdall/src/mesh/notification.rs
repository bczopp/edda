//! Stub für E-Mail-Benachrichtigung an Owner (z. B. bei neuer Mesh-Device-Registrierung).
//! Kein echter E-Mail-Versand; nur Logging, damit der Plan-Punkt „Email-Benachrichtigung“ als Stub abgehakt werden kann.

use tracing::info;
use uuid::Uuid;

/// Benachrichtigt den Owner über ein neu registriertes Device (Stub: nur Logging).
/// Echter E-Mail-Versand wird später ergänzt.
pub fn notify_owner_new_device(owner_id: Uuid, device_id: &str) {
    info!(
        owner_id = %owner_id,
        device_id = %device_id,
        "notify_owner_new_device (stub): would send email to owner about new mesh device"
    );
}
