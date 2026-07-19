use crate::app::Message;
use crate::models::Cycle;
use crate::utils::time::remaining;

use iced::Element;
use iced::widget::{column, container, text};

pub fn view<'a>(title: &'static str, cycle: &'a Cycle) -> Element<'a, Message> {
    container(
        column![
            text(title).size(28),
            text("──────────────"),
            text(format!("Current: {}", cycle.state)),
            text(format!("Time Remaining: {}", remaining(&cycle.expiry))),
        ]
        .spacing(5),
    )
    .style(container::rounded_box)
    .padding(15)
    .width(220)
    .into()
}
