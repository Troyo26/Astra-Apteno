use crate::app::Message;
use crate::models::Cycle;
use crate::ui::components::cycle_background;
use crate::utils::time::{progress, remaining};

use iced::widget::{container, row, stack, text};
use iced::{Element, Fill, color, padding};

fn cycles_content<'a>(title: &'static str, cycle: &'a Cycle) -> Element<'a, Message> {
    let content = row![
        container(text(title)).width(Fill).align_left(Fill),
        container(text(&cycle.state)).width(Fill).center_x(Fill),
        container(text(remaining(&cycle.expiry)))
            .width(Fill)
            .align_right(Fill),
    ];
    content.padding(5).into()
}

pub fn view<'a>(title: &'static str, cycle: &'a Cycle) -> Element<'a, Message> {
    stack!(
        cycle_background::view(progress(&cycle.activation, &cycle.expiry), color!(0, 0, 0)),
        cycles_content(title, cycle)
    )
    .width(500)
    .height(30)
    .into()
}
