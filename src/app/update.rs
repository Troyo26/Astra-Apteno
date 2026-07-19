use crate::api::refresh_connection;
use crate::app::{AppState, ConnectionState, Message};
use iced::Task;
pub fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Refresh => {
            println!("Refresh message received");
            state.connection_state = ConnectionState::Refreshing;

            Task::perform(refresh_connection(), |result| match result {
                Ok(world) => Message::RefreshSucceeded(world),
                Err(err) => {
                    eprintln!("{:#}", err);
                    Message::RefreshFailed
                }
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
