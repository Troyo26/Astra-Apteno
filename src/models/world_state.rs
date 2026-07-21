use crate::models::{Arbitration, ArchonHunt, Cycle, Sortie, VoidTrader};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldState {
    pub sortie: Sortie,
    pub archon_hunt: ArchonHunt,
    pub void_trader: VoidTrader,
    pub earth_cycle: Cycle,
    pub cetus_cycle: Cycle,
    pub cambion_cycle: Cycle,
    pub zariman_cycle: Cycle,
    pub duviri_cycle: Cycle,
    pub arbitration: Arbitration,
}
