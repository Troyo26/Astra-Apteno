// Imports

use crate::app::{AppState, ConnectionState, Message, Tab};
use crate::ui::{archon_hunt, sortie};
use iced::widget::{button, column, container, row, text};
use iced::{Element, Fill};

// Functions
fn status_text(state: &ConnectionState) -> &'static str {
    match state {
        ConnectionState::Refreshing => "Refreshing...",
        ConnectionState::Connected => "Connected",
        ConnectionState::Disconnected => "Disconnected",
    }
}

// View function for Iced
pub fn view(state: &AppState) -> Element<'_, Message> {
    let sortie_widget = if let Some(world) = &state.world_state {
        sortie::view(&world.sortie)
    } else {
        text("No data loaded").into()
    };

    let archon_widget = if let Some(world) = &state.world_state {
        archon_hunt::view(&world.archon_hunt)
    } else {
        text("No data loaded").into()
    };

    let refresh_button = match state.connection_state {
        ConnectionState::Refreshing => button(text("Refreshing...")),
        _ => button(text("↻")).on_press(Message::Refresh),
    };

    let content: Element<'_, Message> = match state.current_tab {
        Tab::Home => column![
            text("Welcome to Astra Apteno"),
            text("Version 0.1"),
            text(status_text(&state.connection_state)),
        ]
        .into(),

        Tab::WorldState => row![sortie_widget, archon_widget,].spacing(20).into(),
    };

    let tab_bar = row![
        button("Home").on_press(Message::SwitchTab(Tab::Home)),
        button("World State").on_press(Message::SwitchTab(Tab::WorldState)),
        refresh_button,
    ]
    .spacing(10);

    let layout = column![
        container(tab_bar)
            .style(container::rounded_box)
            .width(Fill)
            .padding(5),
        container(content).width(Fill).height(Fill).align_left(Fill),
    ];

    container(layout)
        .width(Fill)
        .height(Fill)
        .padding(10)
        .into()
}
