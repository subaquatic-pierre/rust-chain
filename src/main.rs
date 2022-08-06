use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger;

use std::io;

mod app;
mod blockchain;
mod services;
mod storage;

use app::new_app_state;
use blockchain::chain::Chain;

use services::block::register_block_service;
use services::chain::register_chain_service;
use services::transactions::register_transaction_service;

use storage::Storage;

const DIFFICULTY_LEVEL: usize = 3;
const MINER_ADDRESS: &str = "Nebula Miner";
const REWARD: f64 = 10.0;
const SERVER_HOST: (&str, u16) = ("127.0.0.1", 7878);

#[actix_web::main]
async fn main() -> io::Result<()> {
    let blockchain = Chain::new(DIFFICULTY_LEVEL, MINER_ADDRESS, REWARD);
    let storage = Storage {};

    let app_state = new_app_state(blockchain, storage);

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!(
        "Server listening at {:}:{:}...",
        SERVER_HOST.0, SERVER_HOST.1
    );

    // Make new HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(app_state.clone())
            .service(register_transaction_service())
            .service(register_block_service())
            .service(register_chain_service())
    })
    .bind(SERVER_HOST)?
    .run()
    .await
}
