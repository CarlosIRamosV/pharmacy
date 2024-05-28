use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path, ServiceConfig};
use actix_web::{get, post, HttpResponse};

use database::PgPool;

use crate::actions;
use crate::models::{Image, Request, Response};

pub fn load_routes(cfg: &mut ServiceConfig) {
    // General
    cfg.service(new);

    // Specific
    cfg.service(get);
}

#[post("/")]
async fn new(pool: Data<PgPool>, request: Json<Request>) -> Result<Json<Image>, actix_web::Error> {
    let element = actions::create(&pool, request.into_inner()).await?;
    Ok(Json(element))
}

#[get("/{id}")]
async fn get(pool: Data<PgPool>, id: Path<i32>) -> Result<Json<Response>, actix_web::Error> {
    let element = actions::get(&pool, id.into_inner()).await?;
    Ok(Json(Response {
        image: element.get_image(),
    }))
}
