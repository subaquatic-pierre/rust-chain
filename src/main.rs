use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger;

use std::io;

use rust_chain::app::new_app_state;
use rust_chain::services::block::register_block_service;
use rust_chain::services::chain::register_chain_service;
use rust_chain::services::transactions::register_transaction_service;

const SERVER_HOST: (&str, u16) = ("127.0.0.1", 7878);

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app_state = new_app_state();

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
