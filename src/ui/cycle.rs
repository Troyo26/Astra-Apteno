use crate::app::Message;
use crate::models::Cycle;
use crate::utils::time::remaining;

use iced::widget::{Space, column, container, row, text};
use iced::{Element, Fill};

pub fn view<'a>(title: &'static str, cycle: &'a Cycle) -> Element<'a, Message> {
    row![
        text(title),
        Space::new().width(Fill),
        text(&cycle.state),
        Space::new().width(Fill),
        text(remaining(&cycle.expiry)),
    ]
    .width(200)
    .into()
}
