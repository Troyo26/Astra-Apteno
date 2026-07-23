use crate::app::Message;
use crate::ui::components::style;

use iced::widget::{Space, button, row, text};
use iced::{Alignment, Element, Fill};

pub fn view<'a>(
    title: &'a str,
    timer: Option<String>,
    expanded: bool,
    on_press: Message,
) -> Element<'a, Message> {
    let row = if let Some(timer) = timer {
        row![
            text(if expanded { "▼" } else { "▶" }),
            text(title),
            Space::new().width(Fill),
            text(timer),
        ]
    } else {
        row![text(if expanded { "▼" } else { "▶" }), text(title),]
    };

    button(row.align_y(Alignment::Center))
        .style(style::menu_button)
        .on_press(on_press)
        .width(Fill)
        .into()
}
