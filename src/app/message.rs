use crate::app::Tab;
use crate::models::WorldState;
#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    RefreshSucceeded(WorldState),
    RefreshFailed,
    SwitchTab(Tab),
}
