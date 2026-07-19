use crate::app::Message;
use crate::models::{Sortie, SortieVariant};
use chrono::{DateTime, Utc};
use iced::Element;
use iced::widget::{column, container, text};

fn mission_view(index: usize, variant: &SortieVariant) -> Element<'static, Message> {
    container(
        column![
            text(format!("Mission {}", index + 1)).size(20),
            text(format!("•Type: {}", variant.mission_type)),
            text(format!("•Node: {}", variant.node)),
            text(format!("•Modifier: {}", variant.modifier)),
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

fn status(_sortie: &Sortie) -> &'static str {
    "Active"
}

pub fn view(sortie: &Sortie) -> Element<'_, Message> {
    let mut content = column![
        text("Sortie").size(28),
        text("──────────────────────────"),
        text(format!("Status: {}", status(sortie))),
        text(format!("Expires in: {}", expiry_remaining(&sortie.expiry))),
        text(format!("Boss: {}", sortie.boss)),
        text(format!("Faction: {}", sortie.faction)),
        text(""),
    ]
    .spacing(5);

    for (index, variant) in sortie.variants.iter().enumerate() {
        content = content.push(mission_view(index, variant)).push(text(""));
    }

    content.into()
}
