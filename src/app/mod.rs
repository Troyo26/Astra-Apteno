pub mod message;
pub mod state;
pub mod update;
pub mod widget;

use crate::ui::view::view;
use iced::{Subscription, Task, time};
pub use message::*;
pub use state::*;
use std::time::Duration;
pub use update::update;
pub use widget::*;

fn subscription(_state: &AppState) -> Subscription<Message> {
    time::every(Duration::from_secs(60)).map(|_| Message::Refresh)
}

fn init() -> (AppState, Task<Message>) {
    (
        AppState {
            connection_state: ConnectionState::Disconnected,
            world_state: None,
            current_tab: Tab::Home,

            sortie_expanded: false,
            archon_hunt_expanded: false,
            void_trader_expanded: false,
            arbitration_expanded: false,
        },
        Task::done(Message::Refresh),
    )
}

pub fn run() -> iced::Result {
    iced::application(init, update, view)
        .subscription(subscription)
        .run()
}
