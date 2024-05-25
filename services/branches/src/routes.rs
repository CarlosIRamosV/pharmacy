use actix_web::web::{Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put};

use database::PgPool;

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
    pool: Data<PgPool>,
    search: Query<Search>,
) -> Result<Json<Vec<Branch>>, actix_web::Error> {
    let list = actions::get_all(&pool, &search).await?;
    Ok(Json(list))
}

#[post("/")]
async fn new(pool: Data<PgPool>, request: Json<Request>) -> Result<Json<Branch>, actix_web::Error> {
    let element = actions::create(&pool, request.into_inner()).await?;
    Ok(Json(element))
}

#[get("/{id}")]
async fn get(pool: Data<PgPool>, id: Path<i32>) -> Result<Json<Branch>, actix_web::Error> {
    let element = actions::get(&pool, id.into_inner()).await?;
    Ok(Json(element))
}

#[put("/{id}")]
async fn update(
    pool: Data<PgPool>,
    id: Path<i32>,
    update: Json<Update>,
) -> Result<Json<Branch>, actix_web::Error> {
    let element = actions::update(&pool, id.into_inner(), update.into_inner()).await?;
    Ok(Json(element))
}

#[delete("/{id}")]
async fn delete(pool: Data<PgPool>, id: Path<i32>) -> Result<Json<Branch>, actix_web::Error> {
    let element = actions::delete(&pool, id.into_inner()).await?;
    Ok(Json(element))
}
