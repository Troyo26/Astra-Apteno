use crate::app::Message;

use iced::widget::{Space, button, row, text};
use iced::{Alignment, Element, Fill};

pub fn view<'a>(
    title: &'static str,
    timer: String,
    expanded: bool,
    message: Message,
) -> Element<'a, Message> {
    button(
        row![
            text(if expanded { "▼" } else { "▶" }),
            text(title).size(24),
            Space::new().width(Fill),
            text(timer),
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    )
    .style(button::text)
    .on_press(message)
    .into()
}
