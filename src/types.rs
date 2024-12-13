use serde::Serialize; // Import Serialize to convert struct to JSON format
use chrono::{DateTime, Utc}; // Import DateTime and Utc for timestamp

#[derive(Serialize)] // Serialization for struct
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>, // HTTP status code or error
    pub response_time: String,       // example, "1.23 seconds"
    pub timestamp: DateTime<Utc>, 
}
