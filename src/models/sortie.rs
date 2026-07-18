use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Sortie {
    pub expiry: String,
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
