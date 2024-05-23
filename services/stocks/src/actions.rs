use std::error::Error;

use postgres::Row;
use postgres_from_row::FromRow;

use database::PgPool;

use crate::models::{Search, Stock, StockRequest, StockUpdate};
use crate::utils::rows_to_stocks;

pub async fn get_all_stocks(pool: &PgPool, search: &Search) -> Result<Vec<Stock>, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    log::info!("Searching for stocks");

    let stocks =
        if search.product_name.is_some() || search.branch_id.is_some() {
            let mut query = "SELECT * FROM stocks_view WHERE".to_string();
            let mut count = 1;

            if let Some(product_name) = &search.product_name {
                query.push_str(&format!(" product_name LIKE '%{}%'", product_name));
                count += 1;
            }

            if let Some(branch_id) = search.branch_id {
                if count > 1 {
                    query.push_str(" AND");
                }
                query.push_str(&format!(" branch_id = {}", branch_id));
                count += 1;
            }

            conn.interact(move |client| {
                client
                    .query(&query, &[])
                    .map(|rows: Vec<Row>| rows_to_stocks(rows))
            })
                .await?
        } else {
            log::info!("No search parameters provided");
            conn.interact(|client| {
                client
                    .query("SELECT * FROM stocks_view;", &[])
                    .map(|rows: Vec<Row>| rows_to_stocks(rows))
            })
                .await?
        };
    match stocks {
        Ok(stocks) => Ok(stocks),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn get_stock(pool: &PgPool, stock_id: i32) -> Result<Stock, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    let stock = conn
        .interact(move |client| {
            client
                .query_one("SELECT * FROM stocks_view WHERE id = $1;", &[&stock_id])
                .map(|row| Stock::from_row(&row))
        })
        .await?;
    match stock {
        Ok(stock) => Ok(stock),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn create_stock(pool: &PgPool, stock: StockRequest) -> Result<Stock, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    let stock = conn
        .interact(move |client| {
            client
                .query_one(
                    "SELECT * FROM fn_stocks_insert($1, $2, $3);",
                    &[&stock.product_id, &stock.branch_id, &stock.quantity],
                )
                .map(|row| Stock::from_row(&row))
        })
        .await?;
    match stock {
        Ok(stock) => Ok(stock),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn update_stock(
    pool: &PgPool,
    stock_id: i32,
    stock: StockUpdate,
) -> Result<Stock, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    let stock = conn
        .interact(move |client| {
            client
                .query_one(
                    "SELECT * FROM fn_stocks_update($1, $2);",
                    &[&stock_id, &stock.quantity],
                )
                .map(|row| Stock::from_row(&row))
        })
        .await?;
    match stock {
        Ok(stock) => Ok(stock),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn delete_stock(pool: &PgPool, stock_id: i32) -> Result<(), Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    let status = conn
        .interact(move |client| {
            client
                .execute("SELECT * FROM fn_stocks_delete($1);", &[&stock_id])
                .map(|_| ())
        })
        .await?;
    match status {
        Ok(_) => Ok(()),
        Err(err) => Err(Box::new(err)),
    }
}

