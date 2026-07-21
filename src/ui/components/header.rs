use crate::app::{Message, Widget};
use crate::ui::components::style;

use iced::widget::{Space, button, row, text};
use iced::{Alignment, Element, Fill};

pub fn view<'a>(
    title: &'static str,
    timer: impl Into<String>,
    expanded: bool,
    widget: Widget,
) -> Element<'a, Message> {
    button(
        row![
            text(if expanded { "▼" } else { "▶" }),
            text(title),
            Space::new().width(Fill),
            text(timer.into()),
        ]
        .align_y(Alignment::Center),
    )
    .style(style::menu_button)
    .on_press(Message::ToggleWidget(widget))
    .into()
}
