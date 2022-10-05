use actix_web::{
    get, post,
    web::{scope, Data, Json},
    HttpResponse, Scope,
};
use serde::{Deserialize, Serialize};

use crate::app::AppState;

#[derive(Serialize, Deserialize)]
pub struct SetRequest {
    new_value: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ChainRewardResponse {
    value: f64,
}
#[derive(Serialize, Deserialize)]
pub struct ChainDifficultyResponse {
    value: usize,
}

#[post("/set-reward")]
async fn set_chain_reward(app: Data<AppState>, body: Json<SetRequest>) -> HttpResponse {
    let mut chain = app.chain.lock().unwrap();
    chain.set_reward(body.new_value);

    HttpResponse::Ok().json(ChainRewardResponse {
        value: chain.reward(),
    })
}
#[get("/get-reward")]
async fn get_chain_reward(app: Data<AppState>) -> HttpResponse {
    let chain = app.chain.lock().unwrap();

    HttpResponse::Ok().json(ChainRewardResponse {
        value: chain.reward(),
    })
}

#[post("/set-difficulty")]
async fn set_chain_difficulty(app: Data<AppState>, body: Json<SetRequest>) -> HttpResponse {
    let mut chain = app.chain.lock().unwrap();
    chain.set_difficulty(body.new_value as usize);

    HttpResponse::Ok().json(ChainRewardResponse {
        value: chain.reward(),
    })
}

#[get("/get-difficulty")]
async fn get_chain_difficulty(app: Data<AppState>) -> HttpResponse {
    let chain = app.chain.lock().unwrap();

    HttpResponse::Ok().json(ChainDifficultyResponse {
        value: chain.difficulty(),
    })
}

pub fn register_chain_service() -> Scope {
    scope("/chain")
        .service(set_chain_reward)
        .service(set_chain_difficulty)
        .service(get_chain_reward)
        .service(get_chain_difficulty)
}
