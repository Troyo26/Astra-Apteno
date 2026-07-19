use crate::app::Message;
use crate::models::{EventStatus, VoidTrader};
use crate::utils::time::{remaining, status};
use iced::Element;
use iced::widget::{column, text};

pub fn view(trader: &VoidTrader) -> Element<'_, Message> {
    let event_status = status(&trader.activation, &trader.expiry);

    let status_text = match event_status {
        EventStatus::Upcoming => "Coming Soon",
        EventStatus::Active => "Available",
        EventStatus::Expired => "Gone",
    };

    let timer = match event_status {
        EventStatus::Upcoming => format!("Arrives in: {}", remaining(&trader.activation)),

        EventStatus::Active => format!("Leaves in: {}", remaining(&trader.expiry)),

        EventStatus::Expired => "Expired".to_string(),
    };

    let content = column![
        text("Void Trader").size(28),
        text("──────────────────────────"),
        text(format!("Status: {}", status_text)),
        text(timer),
        text(format!("Character: {}", trader.character)),
        text(format!("Location: {}", trader.location)),
        text(""),
    ]
    .spacing(5);

    content.into()
}
