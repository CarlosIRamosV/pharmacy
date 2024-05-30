use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, HttpResponse};

use models::Response;
use server::AppState;

use crate::actions;
use crate::models::{Product, Public, Request, Search, Update};

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
) -> Result<Json<Vec<Public>>, actix_web::Error> {
    let list = actions::get_all(&data.pool, &search).await?;
    Ok(Json(
        list.into_iter()
            .map(|product| product.to_public())
            .collect(),
    ))
}

#[post("/")]
async fn new(data: Data<AppState>, request: Json<Request>) -> HttpResponse {
    let message = actions::create(&data.pool, request.into_inner())
        .await
        .unwrap();
    return if message == "Product created" {
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
async fn get(data: Data<AppState>, id: Path<i32>) -> Result<Json<Product>, actix_web::Error> {
    let element = actions::get(&data.pool, id.into_inner()).await?;
    Ok(Json(element))
}

#[put("/{id}")]
async fn update(data: Data<AppState>, id: Path<i32>, update: Json<Update>) -> HttpResponse {
    let message = actions::update(&data.pool, id.into_inner(), update.into_inner())
        .await
        .unwrap();
    return if message == "Product updated" {
        HttpResponse::build(StatusCode::OK).json(Response {
            status: StatusCode::OK.as_u16(),
            message,
        })
    } else {
        HttpResponse::build(StatusCode::BAD_REQUEST).json(Response {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message,
        })
    };
}

#[delete("/{id}")]
async fn delete(data: Data<AppState>, id: Path<i32>) -> HttpResponse {
    let message = actions::delete(&data.pool, id.into_inner()).await.unwrap();
    return if message == "Product deleted" {
        HttpResponse::build(StatusCode::OK).json(Response {
            status: StatusCode::OK.as_u16(),
            message,
        })
    } else {
        HttpResponse::build(StatusCode::BAD_REQUEST).json(Response {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message,
        })
    };
}
