use crate::app::{Tab, Widget};
use crate::models::WorldState;
#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    RefreshSucceeded(WorldState),
    RefreshFailed,
    SwitchTab(Tab),
    ToggleWidget(Widget),
}
