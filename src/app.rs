use std::sync::Mutex;

use actix_web::middleware::Logger;
use actix_web::web::Data;

use crate::blockchain::blockchain::Chain;
use crate::blockchain::data::TransferData;
use crate::storage::Storage;

pub struct AppState {
    pub app_name: String,
    pub blockchain: Mutex<Chain<TransferData>>,
    pub storage: Mutex<Storage>,
    pub counter: Mutex<i32>,
}

pub fn new_app_state(blockchain: Chain<TransferData>, storage: Storage) -> Data<AppState> {
    let data = Data::new(AppState {
        app_name: String::from("Chain App"),
        blockchain: Mutex::new(blockchain),
        storage: Mutex::new(storage),
        counter: Mutex::new(0),
    });

    data
}

pub fn new_logger() -> Logger {
    Logger::default()
}
