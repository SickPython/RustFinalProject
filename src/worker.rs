use crate::types::WebsiteStatus; // Import WebsiteStatus struct to stor website monitor results
use chrono::Utc; // Import the Utc module for current time
use std::time::{Instant, Duration}; // Import instant to measure the time intervals and timeout handling duration
use ureq::{AgentBuilder, Error}; // Import AgentBuilder to create HTTP clients and Error to handle request erros

pub fn check_website(url: &str, timeout: u64) -> WebsiteStatus {
    let agent = AgentBuilder::new() // Create HTTP agent
        .timeout(Duration::from_secs(timeout)) // Set timeout duration for HTTP request
        .build();

    let start_time = Instant::now(); // Record start time to measrue response time

    let status = match agent.get(url).call() {
        Ok(response) => Ok(response.status()), // If successful, extract HTTP status code
        Err(Error::Transport(transport_err)) => Err(format!("Transport error: {}", transport_err)), // Handle trasnport errors
        Err(Error::Status(code, _)) => Err(format!("HTTP error: {}", code)), // Handle HTTP errors
    };

    let response_time = start_time.elapsed(); // Calculate total time taken for request

    WebsiteStatus { // Construct and return WebsiteStatus with the information gathered from the request
        url: url.to_string(), //
        status,
        response_time: format!("{:.2} seconds", response_time.as_secs_f64()),
        timestamp: Utc::now(),
    }
}
