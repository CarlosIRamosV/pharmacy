use std::error::Error;

use postgres::Row;

use database::{get_error_code, PgPool};

use crate::models::{Product, Request, Search, Update};
use crate::utils::rows_to_products;

pub async fn get_all(pool: &PgPool, search: &Search) -> Result<Vec<Product>, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();

    let products = if search.name.is_some()
        || search.description.is_some()
        || search.min_price.is_some()
        || search.max_price.is_some()
        || search.limit.is_some()
        || search.offset.is_some()
    {
        let mut query = "SELECT * FROM product_view".to_string();
        let mut count = 1;

        if search.name.is_some()
            || search.description.is_some()
            || search.min_price.is_some()
            || search.max_price.is_some()
        {
            query.push_str(" WHERE");
        }

        if let Some(name) = &search.name {
            query.push_str(&format!(" name LIKE '%{}%'", name));
            count += 1;
        }

        if let Some(description) = &search.description {
            if count > 1 {
                query.push_str(" AND");
            }
            query.push_str(&format!(" description LIKE '%{}%'", description));
            count += 1;
        }

        if let Some(min_price) = search.min_price {
            if count > 1 {
                query.push_str(" AND");
            }
            query.push_str(&format!(" price >= {}", min_price));
            count += 1;
        }

        if let Some(max_price) = search.max_price {
            if count > 1 {
                query.push_str(" AND");
            }
            query.push_str(&format!(" price <= {}", max_price));
        }

        if let Some(limit) = search.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = search.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        conn.interact(move |client| {
            client
                .query(&query, &[])
                .map(|rows: Vec<Row>| rows_to_products(rows))
        })
        .await?
    } else {
        conn.interact(|client| {
            client
                .query("SELECT * FROM product_view;", &[])
                .map(|rows: Vec<Row>| rows_to_products(rows))
        })
        .await?
    };
    match products {
        Ok(products) => Ok(products),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn get(pool: &PgPool, id: i32) -> Result<Product, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    let product = conn
        .interact(move |client| {
            client
                .query_one("SELECT * FROM product_view WHERE id = $1;", &[&id])
                .map(|row| Product::from_row(&row))
        })
        .await?;
    match product {
        Ok(product) => Ok(product),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn create(pool: &PgPool, request: Request) -> Result<String, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    let product = conn
        .interact(move |client| {
            client.query_one(
                "CALL add_product($1, $2, $3);",
                &[&request.name, &request.description, &request.price],
            )
        })
        .await?;
    if product.is_err() {
        if get_error_code(&product.err().unwrap()) == 23505 {
            log::info!("Product already exists");
            return Ok("Product already exists".to_string());
        }
    }

    return Ok("Product created".to_string());
}

pub async fn update(pool: &PgPool, id: i32, update: Update) -> Result<String, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    let product = conn
        .interact(move |client| {
            let mut query = "UPDATE product SET".to_string();
            let mut count = 1;

            if let Some(name) = &update.name {
                query.push_str(&format!(" name = '{}'", name));
                count += 1;
            }

            if let Some(description) = &update.description {
                if count > 1 {
                    query.push_str(",");
                }
                query.push_str(&format!(" description = '{}'", description));
                count += 1;
            }

            if let Some(price) = &update.price {
                if count > 1 {
                    query.push_str(",");
                }
                query.push_str(&format!(" price = {}", price));
                count += 1;
            }

            if count > 1 {
                query.push_str(",");
            }

            query.push_str(" updated_at = now()");

            query.push_str(&format!(" WHERE id = {};", id));

            client.query_one(&query, &[])
        })
        .await
        .unwrap();

    log::info!(
        "Product updated {:?}",
        get_error_code(&product.err().unwrap())
    );
    Ok("Product updated".to_string())
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<String, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    let product = conn
        .interact(move |client| client.query_one("DELETE FROM product WHERE id = $1;", &[&id]))
        .await
        .unwrap();
    log::info!(
        "Product deleted {:?}",
        get_error_code(&product.err().unwrap())
    );
    Ok("Product deleted".to_string())
}
