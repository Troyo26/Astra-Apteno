use iced::widget::{button, column, container, row, text};
use iced::{Element, Fill, Task};
use serde::Deserialize;

#[derive(Debug, Clone)]
enum Message {
    Refresh,
    RefreshSucceeded(WorldState),
    RefreshFailed,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Sortie {
    pub boss: String,
    pub faction: String,
    pub variants: Vec<SortieVariant>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SortieVariant {
    #[serde(rename = "missionType")]
    pub mission_type: String,

    pub modifier: String,
    pub node: String,
}

struct AppState {
    connection_state: ConnectionState,
    world_state: Option<WorldState>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConnectionState {
    Refreshing,
    Connected,
    Disconnected,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WorldState {
    pub sortie: Sortie,
}

async fn refresh_connection() -> Result<WorldState, reqwest::Error> {
    let world_state = reqwest::get("https://api.warframestat.us/pc/?language=en")
        .await?
        .json::<WorldState>()
        .await?;

    Ok(world_state)
}

fn update(state: &mut AppState, message: Message) -> Task<Message> {
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
    }
}

fn status_text(state: &ConnectionState) -> &'static str {
    match state {
        ConnectionState::Refreshing => "Refreshing...",
        ConnectionState::Connected => "Connected",
        ConnectionState::Disconnected => "Disconnected",
    }
}

fn init() -> AppState {
    AppState {
        connection_state: ConnectionState::Disconnected,
        world_state: None,
    }
}

fn view(state: &AppState) -> Element<'_, Message> {
    let sortie_text = if let Some(world) = &state.world_state {
        text(&world.sortie.boss)
    } else {
        text("No data loaded")
    };
    let refresh_button = match state.connection_state {
        ConnectionState::Refreshing => button(text("Refreshing...")),
        _ => button(text("Refresh")).on_press(Message::Refresh),
    };
    container(
        column![
            text("Astra Apteno"),
            text("World State"),
            row![text("Status: "), text(status_text(&state.connection_state)),].spacing(10),
            sortie_text,
            refresh_button,
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
