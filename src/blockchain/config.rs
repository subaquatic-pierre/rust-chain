use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ChainConfig {
    pub difficulty: usize,
    pub reward: f64,
}
