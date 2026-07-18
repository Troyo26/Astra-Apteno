pub mod message;
pub mod state;
pub mod update;

use crate::ui::view::view;
use iced::{Subscription, Task, time};
pub use message::*;
pub use state::*;
use std::time::Duration;
pub use update::update;

fn subscription(_state: &AppState) -> Subscription<Message> {
    time::every(Duration::from_secs(60)).map(|_| Message::Refresh)
}

fn init() -> (AppState, Task<Message>) {
    (
        AppState {
            connection_state: ConnectionState::Disconnected,
            world_state: None,
            current_tab: Tab::Home,
        },
        Task::done(Message::Refresh),
    )
}

pub fn run() -> iced::Result {
    iced::application(init, update, view)
        .subscription(subscription)
        .run()
}
