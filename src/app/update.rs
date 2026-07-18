use crate::api::refresh_connection;
use crate::app::{AppState, ConnectionState, Message};
use iced::Task;
pub fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Refresh => {
            state.connection_state = ConnectionState::Refreshing;

            Task::perform(refresh_connection(), |result| match result {
                Ok(world_state) => Message::RefreshSucceeded(world_state),
                Err(_) => Message::RefreshFailed,
            })
        }

        Message::RefreshSucceeded(world_state) => {
            state.connection_state = ConnectionState::Connected;
            state.world_state = Some(world_state);

            Task::none()
        }

        Message::RefreshFailed => {
            state.connection_state = ConnectionState::Disconnected;

            Task::none()
        }

        Message::SwitchTab(tab) => {
            state.current_tab = tab;

            Task::none()
        }
    }
}
