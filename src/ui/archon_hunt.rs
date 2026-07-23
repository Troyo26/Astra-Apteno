use crate::app::{Message, Widget};
use crate::models::{ArchonHunt, ArchonHuntMission};
use crate::ui::components::{divider, header, style};
use crate::utils::time::remaining;
use iced::widget::{column, container, row, text};
use iced::{Element, Fill};

// Functions

fn mission_view<'a>(variant: &'a ArchonHuntMission) -> Element<'a, Message> {
    container(row![
        container(text(&variant.mission_type))
            .width(Fill)
            .align_left(Fill),
        container(text(&variant.node)).width(Fill).align_right(Fill),
    ])
    .padding(8)
    .into()
}

fn status(_archon: &ArchonHunt) -> &'static str {
    "Active"
}

// Compact Widget

fn compact(archon: &ArchonHunt) -> Element<'_, Message> {
    container(header::view(
        "Archon Hunt",
        Some(remaining(&archon.expiry)),
        false,
        Message::ToggleWidget(Widget::ArchonHunt),
    ))
    .width(500)
    .style(style::widget)
    .into()
}

// Expanded Widget

fn expanded_widget(archon: &ArchonHunt) -> Element<'_, Message> {
    let mut content = column![
        header::view(
            "Archon Hunt",
            Some(remaining(&archon.expiry)),
            true,
            Message::ToggleWidget(Widget::ArchonHunt),
        ),
        divider::view(),
        container(row![
            container(text(format!("{}", status(archon))))
                .width(Fill)
                .align_left(Fill),
            container(text(format!("{}", archon.boss)))
                .width(Fill)
                .center_x(Fill),
            container(text(format!("{}", archon.faction)))
                .width(Fill)
                .align_right(Fill),
        ])
        .padding(8),
    ];

    for variant in &archon.missions {
        content = content.push(divider::view());
        content = content.push(mission_view(variant));
    }

    container(content).width(500).style(style::widget).into()
}

// View Function

pub fn view(archon: &ArchonHunt, expanded: bool) -> Element<'_, Message> {
    if expanded {
        expanded_widget(archon)
    } else {
        compact(archon)
    }
}
