#[derive(Debug, Clone)]
enum Message {
    Refresh,
}
struct AppState {
    connection_state: ConnectionState,
}
enum ConnectionState {
    Refreshing,
    Connected,
    Disconnected,
}
fn update(: &mut bool, message: Message) {
    match message {
        Message::Connect => *connected = true,
    }
}

use iced::widget::{button, column, container, row, text};
use iced::{Element, Fill};

fn status_text(status: AppState) -> &'static str {}
fn view(AppState { connected }: &mut bool) -> Element<'_, Message> {
    container(
        column![
            text("Astra Apteno"),
            text("World State"),
            row![text("Status: "), text(status_text(*status)),].spacing(10),
            button(text("Refresh")).on_press(Message::Refresh)),
        ]
        .spacing(20),
    )
    .center_y(Fill)
    .center_x(Fill)
    .padding(10)
    .into()
}

pub fn main() -> iced::Result {
    iced::run(update, view)
}
