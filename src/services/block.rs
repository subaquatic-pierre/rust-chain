use actix_web::web;

use actix_web::{
    get, post,
    web::{scope, Data, Json},
    HttpResponse, Scope,
};
use serde::{Deserialize, Serialize};

use crate::{app::AppState, blockchain::block::Block};

#[post("/mine-new-block")]
async fn mine_new_block(app: Data<AppState>) -> HttpResponse {
    let mut blockchain = app.blockchain.lock().unwrap();
    let block = blockchain.mine_new_block();
    HttpResponse::Ok().json(block)
}

#[get("/list-blocks")]
async fn list_blocks(app: Data<AppState>) -> HttpResponse {
    let blockchain = app.blockchain.lock().unwrap();
    let blocks = blockchain.blocks();
    HttpResponse::Ok().json(blocks)
}

pub fn register_block_service() -> Scope {
    scope("/block").service(mine_new_block).service(list_blocks)
}
