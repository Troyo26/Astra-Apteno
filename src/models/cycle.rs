use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cycle {
    pub activation: String,
    pub expiry: String,
    pub state: String,
    pub time_left: String,

    pub is_day: Option<bool>,
    pub is_corpus: Option<bool>,
    pub is_cetus: Option<bool>,
}
