use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Cycle {
    pub activation: String,
    pub expiry: String,
    pub state: String,

    pub is_day: Option<bool>,
    pub is_corpus: Option<bool>,
    pub is_cetus: Option<bool>,
}
