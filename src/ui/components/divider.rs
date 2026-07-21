// divider.rs

use crate::ui::components::style;
use iced::widget::container;
use iced::{Element, Fill};

use crate::app::Message;

pub fn view() -> Element<'static, Message> {
    container("")
        .height(2)
        .width(Fill)
        .style(style::divider)
        .into()
}
