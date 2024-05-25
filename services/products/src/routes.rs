use actix_web::{delete, get, post, put, web};

use database::PgPool;

use crate::actions;
use crate::models::{Product, Request, Search, Update};

///
/// Load the routes for the application
///
pub fn load_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_products);
    cfg.service(create_product);
    cfg.service(get_product);
    cfg.service(update_product);
    cfg.service(delete_product);
}

#[get("/")]
async fn get_products(pool: web::Data<PgPool>, search: web::Query<Search>) -> Result<web::Json<Vec<Product>>, actix_web::Error> {
    let products = actions::get_all_products(&pool, &search).await?;
    Ok(web::Json(products))
}

#[post("/")]
async fn create_product(pool: web::Data<PgPool>, product: web::Json<Request>) -> Result<web::Json<Product>, actix_web::Error> {
    let product = actions::create_product(&pool, product.into_inner()).await?;
    Ok(web::Json(product))
}

#[get("/{id}")]
async fn get_product(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<web::Json<Product>, actix_web::Error> {
    let product = actions::get_product(&pool, id.into_inner()).await?;
    Ok(web::Json(product))
}

#[put("/{id}")]
async fn update_product(pool: web::Data<PgPool>, id: web::Path<i32>, product: web::Json<Update>) -> Result<web::Json<Product>, actix_web::Error> {
    let product = actions::update_product(&pool, id.into_inner(), product.into_inner()).await?;
    Ok(web::Json(product))
}

#[delete("/{id}")]
async fn delete_product(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<web::Json<Product>, actix_web::Error> {
    let product = actions::delete_product(&pool, id.into_inner()).await?;
    Ok(web::Json(product))
}