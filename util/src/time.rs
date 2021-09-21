//! time module
pub use chrono::{DateTime, TimeZone, Utc};

/// Returns the number of non-leap milliseconds since January 1, 1970 0:00:00 UTC
/// (UNIX timestamp).
fn unix_epoch_ms() -> u64 {
    let now: DateTime<Utc> = Utc::now();

    now.timestamp_millis() as u64
}

