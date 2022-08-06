use std::sync::Mutex;

use actix_web::web::Data;

use crate::blockchain::chain::Chain;
use crate::storage::Storage;

pub struct AppState {
    pub app_name: String,
    pub chain: Mutex<Chain>,
    pub storage: Mutex<Storage>,
    pub counter: Mutex<i32>,
}

pub fn new_app_state(chain: Chain, storage: Storage) -> Data<AppState> {
    let data = Data::new(AppState {
        app_name: String::from("Blockchain App"),
        chain: Mutex::new(chain),
        storage: Mutex::new(storage),
        counter: Mutex::new(0),
    });

    data
}
