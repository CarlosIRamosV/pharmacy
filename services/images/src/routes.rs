use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, ServiceConfig};
use actix_web::{post, HttpResponse};

use server::AppState;

use crate::actions;
use crate::models::Request;

pub fn load_routes(cfg: &mut ServiceConfig) {
    // General
    cfg.service(new);
}

#[post("/")]
async fn new(data: Data<AppState>, request: Json<Request>) -> HttpResponse {
    let message = actions::create(&data.pool, request.into_inner())
        .await
        .unwrap();
    return if message == "Image created" {
        HttpResponse::build(StatusCode::CREATED).json(message)
    } else {
        HttpResponse::build(StatusCode::BAD_REQUEST).json(message)
    };
}
