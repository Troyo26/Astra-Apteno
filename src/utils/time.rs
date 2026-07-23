use chrono::{DateTime, Utc};

use crate::models::EventStatus;

pub fn progress(activation: &str, expiry: &str) -> f32 {
    let start = DateTime::parse_from_rfc3339(activation)
        .unwrap()
        .with_timezone(&Utc);

    let end = DateTime::parse_from_rfc3339(expiry)
        .unwrap()
        .with_timezone(&Utc);

    let now = Utc::now();

    let total = (end - start).num_seconds() as f32;
    let elapsed = (now - start).num_seconds() as f32;

    (elapsed / total).clamp(0.0, 1.0)
}

pub fn remaining(date: &str) -> String {
    match DateTime::parse_from_rfc3339(date) {
        Ok(date) => {
            let date = date.with_timezone(&Utc);

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

        Err(_) => "N/A".to_string(),
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
