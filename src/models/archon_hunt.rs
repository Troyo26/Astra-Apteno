use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ArchonHunt {
    pub expiry: String,
    pub boss: String,
    pub faction: String,
    pub missions: Vec<ArchonHuntMission>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ArchonHuntMission {
    #[serde(rename = "type")]
    pub mission_type: String,

    pub node: String,
}
