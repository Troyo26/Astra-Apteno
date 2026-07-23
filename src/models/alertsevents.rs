use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AlertsEvents {
    pub events: Vec<Activity>,
    pub alerts: Vec<Activity>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Activity {
    pub id: String,
    pub activation: String,
    pub expiry: String,
    pub description: String,
    pub tooltip: String,
    pub node: String,
    pub rewards: Vec<Reward>,

    #[serde(rename = "interimSteps", default)]
    pub interim_steps: Vec<InterimStep>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Reward {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InterimStep {
    pub goal: u32,
    pub reward: Reward,
}
