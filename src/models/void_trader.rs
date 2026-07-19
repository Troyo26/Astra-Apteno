use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct VoidTrader {
    pub activation: String,
    pub expiry: String,
    pub character: String,
    pub location: String,
}
