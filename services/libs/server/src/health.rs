use actix_web::web::Data;
use actix_web::{get, HttpResponse};
use serde::Serialize;

use crate::AppState;

#[derive(Debug, Serialize)]
struct Health {
    app_name: String,
    status: String,
}

#[get("/health")]
async fn health_state(data: Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(Health {
        app_name: data.app_name.clone(),
        status: "UP".to_string(),
    })
}
