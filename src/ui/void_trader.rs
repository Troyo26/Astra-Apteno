use crate::app::{Message, Widget};
use crate::models::{EventStatus, VoidTrader};
use crate::ui::{divider, header, style};
use crate::utils::time::{remaining, status};
use iced::widget::{column, container, row, text};
use iced::{Element, Fill};

// Functions

fn current_status(baro: &VoidTrader) -> String {
    let event_status = status(&baro.activation, &baro.expiry);

    match event_status {
        EventStatus::Upcoming => {
            format!("Arrives in {}", remaining(&baro.activation))
        }

        EventStatus::Active => {
            format!("Leaves in {}", remaining(&baro.expiry))
        }

        EventStatus::Expired => "Expired".to_string(),
    }
}

fn status_text(baro: &VoidTrader) -> &'static str {
    match status(&baro.activation, &baro.expiry) {
        EventStatus::Upcoming => "Coming Soon",
        EventStatus::Active => "In Relay",
        EventStatus::Expired => "On the Move",
    }
}

// Compacted Widget

fn compact(baro: &VoidTrader) -> Element<'_, Message> {
    container(header::view(
        "Void Trader",
        current_status(baro),
        false,
        Widget::VoidTrader,
    ))
    .width(500)
    .style(style::widget)
    .into()
}

// Expanded Widget

fn expanded_widget(baro: &VoidTrader) -> Element<'_, Message> {
    let content = column![
        header::view(
            "Void Trader",
            current_status(baro),
            true,
            Widget::VoidTrader,
        ),
        divider::view(),
        container(row![
            container(text(status_text(baro)))
                .width(Fill)
                .align_left(Fill),
            container(text(&baro.character)).width(Fill).center_x(Fill),
            container(text(&baro.location))
                .width(Fill)
                .align_right(Fill),
        ])
        .padding(8),
    ];

    container(content).width(500).style(style::widget).into()
}

// View Function
pub fn view(baro: &VoidTrader, expanded: bool) -> Element<'_, Message> {
    if expanded {
        expanded_widget(baro)
    } else {
        compact(baro)
    }
}
