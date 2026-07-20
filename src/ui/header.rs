use crate::app::{Message, Widget};
use iced::widget::{Space, button, container, row, text};
use iced::{Alignment, Element, Fill};

pub fn view<'a>(
    title: &'static str,
    timer: String,
    expanded: bool,
    widget: Widget,
) -> Element<'a, Message> {
    button(
        container(
            row![
                text(if expanded { "▼" } else { "▶" }),
                text(title),
                Space::new().width(Fill),
                text(timer),
            ]
            .align_y(Alignment::Center),
        )
        .padding(2),
    )
    .on_press(Message::ToggleWidget(widget))
    .style(button::text)
    .into()
}
