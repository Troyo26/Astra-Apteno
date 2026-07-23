use crate::app::Widget;
use crate::models::WorldState;
use std::collections::HashSet;

pub struct AppState {
    pub connection_state: ConnectionState,
    pub world_state: Option<WorldState>,
    pub current_tab: Tab,

    pub expanded_widgets: HashSet<Widget>,
    pub expanded_activities: HashSet<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Refreshing,
    Connected,
    Disconnected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Home,
    WorldState,
}
