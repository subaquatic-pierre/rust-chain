use std::sync::Mutex;

use actix_web::web::Data;

use crate::blockchain::{chain::Chain, config::ChainConfig};
use crate::storage::Storage;

const DIFFICULTY_LEVEL: usize = 3;
const REWARD: f64 = 10.0;
const MINER_ADDRESS: &str = "Nebula Miner";

pub struct AppState {
    pub app_name: String,
    pub chain: Mutex<Chain>,
    pub storage: Mutex<Storage>,
    pub counter: Mutex<i32>,
}

pub fn new_app_state() -> Data<AppState> {
    let config = ChainConfig {
        difficulty: DIFFICULTY_LEVEL,
        reward: REWARD,
    };
    let chain = Chain::new(config, MINER_ADDRESS);
    let storage = Storage {};

    Data::new(AppState {
        app_name: String::from("Blockchain App"),
        chain: Mutex::new(chain),
        storage: Mutex::new(storage),
        counter: Mutex::new(0),
    })
}
