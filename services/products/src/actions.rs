use std::error::Error;
use postgres::Row;
use postgres_from_row::FromRow;
use database::PgPool;
use crate::models::{Product, Request, Search, Update};
use crate::utils::rows_to_stocks;

pub async fn get_all_products(pool: &PgPool, search: &Search) -> Result<Vec<Product>, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();

    let stocks = if search.name.is_some() || search.branch_id.is_some() || search.min_price.is_some() || search.max_price.is_some() || search.limit.is_some() || search.offset.is_some() {

        let mut query = "SELECT * FROM products_view".to_string();
        let mut count = 1;

        if search.name.is_some() || search.branch_id.is_some() || search.min_price.is_some() || search.max_price.is_some() {
            query.push_str(" WHERE");
        }

        if let Some(name) = &search.name {
            query.push_str(&format!(" name LIKE '%{}%'", name));
            count += 1;
        }

        if let Some(branch_id) = search.branch_id {
            if count > 1 {
                query.push_str(" AND");
            }
            query.push_str(&format!(" branch_id = {}", branch_id));
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
                .map(|rows: Vec<Row>| rows_to_stocks(rows))
        })
            .await?
    } else {
        conn.interact(|client| {
            client
                .query("SELECT * FROM products_view;", &[])
                .map(|rows: Vec<Row>| rows_to_stocks(rows))
        })
            .await?
    };
    match stocks {
        Ok(stocks) => Ok(stocks),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn get_product(pool: &PgPool, product_id: i32) -> Result<Product, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    let product = conn
        .interact(move |client| {
            client
                .query_one("SELECT * FROM products_view WHERE id = $1;", &[&product_id])
                .map(|row| Product::from_row(&row))
        })
        .await?;
    match product {
        Ok(product) => Ok(product),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn create_product(pool: &PgPool, product: Request) -> Result<Product, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    let product = conn
        .interact(move |client| {
            client
                .query_one(
                    "SELECT * FROM fn_products_insert($1, $2, $3);",
                    &[&product.name, &product.description, &product.price],
                )
                .map(|row| Product::from_row(&row))
        })
        .await?;
    match product {
        Ok(product) => Ok(product),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn update_product(pool: &PgPool, product_id: i32, product: Update) -> Result<Product, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    let product = conn
        .interact(move |client| {

            let mut query = "UPDATE products SET".to_string();
            let mut count = 1;

            if let Some(name) = &product.name {
                query.push_str(&format!(" name = '{}'", name));
                count += 1;
            }

            if let Some(description) = &product.description {
                if count > 1 {
                    query.push_str(",");
                }
                query.push_str(&format!(" description = '{}'", description));
                count += 1;
            }

            if let Some(price) = &product.price {
                if count > 1 {
                    query.push_str(",");
                }
                query.push_str(&format!(" price = {}", price));
            }

            query.push_str(", updated_at = now()");

            query.push_str(&format!(" WHERE id = {}", product_id));

            query.push_str(" RETURNING *;");


            client
                .query_one(&query, &[])
                .map(|row| Product::from_row(&row))
        })
        .await?;
    match product {
        Ok(product) => Ok(product),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn delete_product(pool: &PgPool, product_id: i32) -> Result<Product, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    let product = conn
        .interact(move |client| {
            client
                .query_one("DELETE FROM products WHERE id = $1 RETURNING *;", &[&product_id])
                .map(|row| Product::from_row(&row))
        })
        .await?;
    match product {
        Ok(product) => Ok(product),
        Err(err) => Err(Box::new(err)),
    }
}
