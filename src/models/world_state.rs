use crate::models::{ArchonHunt, Sortie};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct WorldState {
    pub sortie: Sortie,

    #[serde(rename = "archonHunt")]
    pub archon_hunt: ArchonHunt,
}
