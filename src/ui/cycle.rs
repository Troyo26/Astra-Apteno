use crate::app::Message;
use crate::models::Cycle;
use crate::utils::time::remaining;

use iced::widget::{container, row, text};
use iced::{Element, Fill};

pub fn view<'a>(title: &'static str, cycle: &'a Cycle) -> Element<'a, Message> {
    row![
        container(text(title)).width(Fill).align_left(Fill),
        container(text(&cycle.state)).width(Fill).center_x(Fill),
        container(text(remaining(&cycle.expiry)))
            .width(Fill)
            .align_right(Fill),
    ]
    .into()
}
