use crate::app::{Message, Widget};
use crate::models::{Sortie, SortieVariant};
use crate::utils::time::remaining;
use iced::widget::{Space, button, column, container, row, text};
use iced::{Alignment, Element, Fill};

// Functions
fn mission_view(index: usize, variant: &SortieVariant) -> Element<'static, Message> {
    container(row![
        text(format!("{}", variant.mission_type)),
        Space::new().width(Fill),
        text(format!("{}", variant.node)),
        Space::new().width(Fill),
        text(format!("{}", variant.modifier)),
    ])
    .style(container::rounded_box)
    .into()
}

fn status(_sortie: &Sortie) -> &'static str {
    "Active"
}

// Compact Widget

fn compact(sortie: &Sortie) -> Element<'_, Message> {
    let timer = remaining(&sortie.expiry);

    button(
        container(
            row![
                text("▶"),
                text("Sortie"),
                Space::new().width(Fill),
                text(timer),
            ]
            .align_y(Alignment::Center),
        )
        .style(container::rounded_box)
        .padding(10),
    )
    .width(500)
    .style(button::text)
    .on_press(Message::ToggleWidget(Widget::Sortie))
    .into()
}

// Expanded Widget

fn expanded_widget(sortie: &Sortie) -> Element<'_, Message> {
    let mut content = column![
        button(
            container(
                row![
                    text("▼"),
                    text("Sortie"),
                    Space::new().width(Fill),
                    text(remaining(&sortie.expiry)),
                ]
                .align_y(Alignment::Center),
            )
            .style(container::rounded_box)
            .padding(10),
        )
        .style(button::text)
        .on_press(Message::ToggleWidget(Widget::Sortie)),
        row![
            text(format!("Status: {}", status(sortie))),
            Space::new().width(Fill),
            text(format!("Boss: {}", sortie.boss)),
            Space::new().width(Fill),
            text(format!("Faction: {}", sortie.faction)),
        ]
    ]
    .spacing(5);

    for (index, variant) in sortie.variants.iter().enumerate() {
        content = content.push(mission_view(index, variant)).push(text(""));
    }

    content.width(500).into()
}

// View Function
pub fn view(sortie: &Sortie, expanded: bool) -> Element<'_, Message> {
    if expanded {
        expanded_widget(sortie)
    } else {
        compact(sortie)
    }
}
