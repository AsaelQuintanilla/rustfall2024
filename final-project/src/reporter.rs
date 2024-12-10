use crate::models::WebsiteStatus;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn report_statuses(statuses: &[WebsiteStatus]) {
    for status in statuses {
        let timestamp = status.timestamp
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        println!(
            "URL: {}, Status: {:?}, Response Time: {:?}, Timestamp: {}",
            status.url,
            status.status,
            status.response_time,
            timestamp
        );
    }
}


