use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Arbitration {
    pub expiry: String,
    pub node: String,
    #[serde(rename = "type")]
    pub mission_type: String,

    #[serde(rename = "enemy")]
    pub enemy_type: String,
}
