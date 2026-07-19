use chrono::{DateTime, Utc};

use crate::models::EventStatus;

pub fn remaining(date: &str) -> String {
    let date = DateTime::parse_from_rfc3339(date)
        .unwrap()
        .with_timezone(&Utc);

    let remaining = date - Utc::now();

    if remaining.num_seconds() <= 0 {
        return "Expired".to_string();
    }

    let days = remaining.num_days();
    let hours = remaining.num_hours() % 24;
    let minutes = remaining.num_minutes() % 60;

    if days > 0 {
        format!("{days}d {hours}h")
    } else {
        format!("{hours}h {minutes}m")
    }
}

#[allow(dead_code)]
pub fn is_active(activation: &str, expiry: &str) -> bool {
    status(activation, expiry) == EventStatus::Active
}

pub fn status(activation: &str, expiry: &str) -> EventStatus {
    let now = Utc::now();

    let activation = DateTime::parse_from_rfc3339(activation)
        .unwrap()
        .with_timezone(&Utc);

    let expiry = DateTime::parse_from_rfc3339(expiry)
        .unwrap()
        .with_timezone(&Utc);

    if now < activation {
        EventStatus::Upcoming
    } else if now < expiry {
        EventStatus::Active
    } else {
        EventStatus::Expired
    }
}
