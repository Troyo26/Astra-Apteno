use crate::app::{Message, Widget};
use crate::models::Arbitration;
use crate::ui::components::{divider, header, style};
use crate::utils::time::remaining;
use iced::widget::{column, container, row, text};
use iced::{Element, Fill};

// Functions

fn info_row<'a>(arbitration: &'a Arbitration) -> Element<'a, Message> {
    container(row![
        container(text(&arbitration.mission_type))
            .width(Fill)
            .align_left(Fill),
        container(text(&arbitration.enemy_type))
            .width(Fill)
            .center_x(Fill),
        container(text(&arbitration.node))
            .width(Fill)
            .align_right(Fill),
    ])
    .padding(8)
    .into()
}

/* fn status(arbitration: &Arbitration) -> &'static str {
    if arbitration.mission_type == "Unknown" {
        "Inactive"
    } else {
        "Active"
    }
} */

// Compact Widget

fn compact(_arbitration: &Arbitration) -> Element<'_, Message> {
    container(header::view(
        "Arbitration",
        Some(remaining(&_arbitration.expiry)),
        false,
        Message::ToggleWidget(Widget::Arbitration),
    ))
    .width(500)
    .style(style::widget)
    .into()
}

// Expanded Widget

fn expanded_widget(_arbitration: &Arbitration) -> Element<'_, Message> {
    let mut content = column![header::view(
        "Arbitration",
        Some(remaining(&_arbitration.expiry)),
        true,
        Message::ToggleWidget(Widget::Arbitration),
    )];

    content = content.push(divider::view());
    content = content.push(info_row(_arbitration));

    container(content).width(500).style(style::widget).into()
}

// View Function

pub fn view(_arbitration: &Arbitration, expanded: bool) -> Element<'_, Message> {
    if expanded {
        expanded_widget(_arbitration)
    } else {
        compact(_arbitration)
    }
}
