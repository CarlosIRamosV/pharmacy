use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, HttpResponse};
use models::Response;

use server::AppState;

use crate::actions;
use crate::models::{Branch, Request, Search, Update};

pub fn load_routes(cfg: &mut ServiceConfig) {
    // General
    cfg.service(get_list);
    cfg.service(new);

    // Specific
    cfg.service(get);
    cfg.service(update);
    cfg.service(delete);
}

#[get("/")]
async fn get_list(
    data: Data<AppState>,
    search: Query<Search>,
) -> Result<Json<Vec<Branch>>, actix_web::Error> {
    let list = actions::get_all(&data.pool, &search.into_inner()).await?;
    Ok(Json(list))
}

#[post("/")]
async fn new(data: Data<AppState>, request: Json<Request>) -> HttpResponse {
    let message = actions::create(&data.pool, request.into_inner())
        .await
        .unwrap();

    return if message == "Branch created" {
        HttpResponse::build(StatusCode::CREATED).json(Response {
            status: StatusCode::CREATED.as_u16(),
            message,
        })
    } else {
        HttpResponse::build(StatusCode::BAD_REQUEST).json(Response {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message,
        })
    };
}

#[get("/{id}")]
async fn get(data: Data<AppState>, id: Path<i32>) -> Result<Json<Branch>, actix_web::Error> {
    let element = actions::get(&data.pool, id.into_inner()).await?;
    Ok(Json(element))
}

#[put("/{id}")]
async fn update(
    data: Data<AppState>,
    id: Path<i32>,
    update: Json<Update>,
) -> Result<Json<Branch>, actix_web::Error> {
    let element = actions::update(&data.pool, id.into_inner(), update.into_inner()).await?;
    Ok(Json(element))
}

#[delete("/{id}")]
async fn delete(data: Data<AppState>, id: Path<i32>) -> Result<Json<Branch>, actix_web::Error> {
    let element = actions::delete(&data.pool, id.into_inner()).await?;
    Ok(Json(element))
}
