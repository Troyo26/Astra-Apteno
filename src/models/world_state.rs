use crate::models::Sortie;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct WorldState {
    pub sortie: Sortie,
}
