#[derive(Debug, Clone)]
enum Message {
    Refresh,
    RefreshSucceeded,
    RefreshFailed,
}

use std::time::Duration;

async fn fake_refresh() {
    std::thread::sleep(std::time::Duration::from_secs(2));
}

struct AppState {
    connection_state: ConnectionState,
}

enum ConnectionState {
    Refreshing,
    Connected,
    Disconnected,
}

use iced::widget::{button, column, container, row, text};
use iced::{Element, Fill, Task};

fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Refresh => {
            state.connection_state = ConnectionState::Refreshing;

            Task::perform(fake_refresh(), |_| Message::RefreshSucceeded)
        }

        Message::RefreshSucceeded => {
            state.connection_state = ConnectionState::Connected;

            Task::none()
        }

        Message::RefreshFailed => {
            state.connection_state = ConnectionState::Disconnected;

            Task::none()
        }
    }
}

fn status_text(state: &ConnectionState) -> &'static str {
    match state {
        ConnectionState::Refreshing => "Refreshing",
        ConnectionState::Connected => "Connected",
        ConnectionState::Disconnected => "Disconnected",
    }
}

fn init() -> AppState {
    AppState {
        connection_state: ConnectionState::Disconnected,
    }
}

fn view(state: &AppState) -> Element<'_, Message> {
    container(
        column![
            text("Astra Apteno"),
            text("World State"),
            row![text("Status: "), text(status_text(&state.connection_state)),].spacing(10),
            button(text("Refresh")).on_press(Message::Refresh),
        ]
        .spacing(20),
    )
    .center_y(Fill)
    .center_x(Fill)
    .padding(10)
    .into()
}

pub fn main() -> iced::Result {
    iced::application(init, update, view).run()
}
