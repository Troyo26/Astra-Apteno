// Imports

use crate::app::{AppState, ConnectionState, Message, Tab};
use crate::ui::{archon_hunt, cycle, divider, sortie, style, void_trader};
use iced::widget::{button, column, container, row, scrollable, text};
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
    let world_widgets: Element<'_, Message> = if let Some(world) = &state.world_state {
        // World Cycles(Earth, Cetus, Cambion Drift, Zariman)
        let cycles = container(
            column![
                cycle::view("Earth", &world.earth_cycle),
                divider::view(),
                cycle::view("Cetus", &world.cetus_cycle),
                divider::view(),
                cycle::view("Cambion", &world.cambion_cycle),
                divider::view(),
                cycle::view("Zariman", &world.zariman_cycle),
                divider::view(),
                cycle::view("Duviri", &world.duviri_cycle),
            ]
            .spacing(5),
        )
        .padding(8)
        .width(500)
        .style(style::widget);
        // Event Widgets for sortie, archon hunt and void trader(baro)
        let events = column![
            sortie::view(&world.sortie, state.sortie_expanded,),
            archon_hunt::view(&world.archon_hunt, state.archon_hunt_expanded),
            void_trader::view(&world.void_trader, state.void_trader_expanded),
        ];

        column![cycles, events,].into()
    } else {
        text("No data loaded").into()
    };

    // Refresh button
    let refresh_button = match state.connection_state {
        ConnectionState::Refreshing => button(text("Refreshing...")),
        _ => button(text("↻")).on_press(Message::Refresh),
    };

    // Element that contains the current tab content
    let content: Element<'_, Message> = match state.current_tab {
        Tab::Home => column![
            text("Welcome to Astra Apteno"),
            text("Version 0.1"),
            text(status_text(&state.connection_state)),
        ]
        .into(),

        Tab::WorldState => world_widgets,
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
        scrollable(container(content).width(Fill).height(Fill).align_left(Fill))
    ];

    container(layout)
        .width(Fill)
        .height(Fill)
        .padding(10)
        .into()
}
