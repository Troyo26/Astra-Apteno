use crate::models::WorldState;

pub struct AppState {
    pub connection_state: ConnectionState,
    pub world_state: Option<WorldState>,
    pub current_tab: Tab,
    pub sortie_expanded: bool,
    pub archon_hunt_expanded: bool,
    pub void_trader_expanded: bool,
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
