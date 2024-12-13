use crate::types::WebsiteStatus;
use chrono::Utc;
use std::time::{Instant, Duration};
use ureq::{AgentBuilder, Error};

pub fn check_website(url: &str, timeout: u64) -> WebsiteStatus {
    let agent = AgentBuilder::new()
        .timeout(Duration::from_secs(timeout))
        .build();

    let start_time = Instant::now();

    let status = match agent.get(url).call() {
        Ok(response) => Ok(response.status()),
        Err(Error::Transport(transport_err)) => Err(format!("Transport error: {}", transport_err)),
        Err(Error::Status(code, _)) => Err(format!("HTTP error: {}", code)),
    };

    let response_time = start_time.elapsed();

    WebsiteStatus {
        url: url.to_string(),
        status,
        response_time: format!("{:.2} seconds", response_time.as_secs_f64()),
        timestamp: Utc::now(),
    }
}
