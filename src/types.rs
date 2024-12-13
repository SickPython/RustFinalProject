use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Serialize)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>, // HTTP status code or error
    pub response_time: String,       // e.g., "1.23 seconds"
    pub timestamp: DateTime<Utc>,
}
