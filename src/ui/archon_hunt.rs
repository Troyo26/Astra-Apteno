use crate::app::Message;
use crate::models::{ArchonHunt, ArchonHuntMission};
use chrono::{DateTime, Utc};
use iced::widget::{column, container, text};
use iced::{Element, Fill};

fn mission_view(index: usize, variant: &ArchonHuntMission) -> Element<'static, Message> {
    container(
        column![
            text(format!("Mission {}", index + 1)).size(20),
            text(format!("•Type: {}", variant.mission_type)),
            text(format!("•Node: {}", variant.node)),
        ]
        .spacing(3),
    )
    .style(container::rounded_box)
    .padding(15)
    .width(300)
    .into()
}

fn expiry_remaining(expiry: &str) -> String {
    let expiry = DateTime::parse_from_rfc3339(expiry)
        .unwrap()
        .with_timezone(&Utc);
    let remaining = expiry - Utc::now();
    if remaining.num_seconds() <= 0 {
        return "Expired".to_string();
    }

    let hours = remaining.num_hours();
    let minutes = remaining.num_minutes() % 60;

    format!("{}h {}m", hours, minutes)
}

fn status(_archon: &ArchonHunt) -> &'static str {
    "Active"
}

pub fn view(archon: &ArchonHunt) -> Element<'_, Message> {
    let mut content = column![
        text("Archon Hunt").size(28),
        text("──────────────────────────"),
        text(format!("Status: {}", status(archon))),
        text(format!("Expires in: {}", expiry_remaining(&archon.expiry))),
        text(format!("Boss: {}", archon.boss)),
        text(format!("Faction: {}", archon.faction)),
        text(""),
    ]
    .spacing(5);

    for (index, mission) in archon.missions.iter().enumerate() {
        content = content.push(mission_view(index, mission)).push(text(""));
    }
    println!("{:#?}", archon);
    content.into()
}
