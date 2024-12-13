use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub urls: Vec<String>,         // List of website URLs
    pub num_threads: usize,        // Number of worker threads
    pub timeout: u64,              // Request timeout in seconds
    pub max_retries: u8,           // Max retries per website
}

impl Config {
    pub fn new(path: &str) -> Result<Self, String> {
        let config_data = fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&config_data).map_err(|e| e.to_string())
    }
}
