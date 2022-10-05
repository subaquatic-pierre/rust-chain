use actix_web::{
    get, post,
    web::{scope, Data},
    HttpResponse, Scope,
};

use crate::app::AppState;

#[post("/mine-new-block")]
async fn mine_new_block(app: Data<AppState>) -> HttpResponse {
    let mut chain = app.chain.lock().unwrap();
    let block = chain.mine_new_block();
    HttpResponse::Ok().json(block)
}

#[get("/list-blocks")]
async fn list_blocks(app: Data<AppState>) -> HttpResponse {
    let chain = app.chain.lock().unwrap();
    let blocks = chain.blocks();
    HttpResponse::Ok().json(blocks)
}

pub fn register_block_service() -> Scope {
    scope("/block").service(mine_new_block).service(list_blocks)
}
