use std::error::Error;

use postgres::Row;
use postgres_from_row::FromRow;

use database::PgPool;

use crate::models::{Branch, Search, StockRequest, StockUpdate};
use crate::utils::rows_to_stocks;

pub async fn get_all_branches(pool: &PgPool, search: &Search) -> Result<Vec<Branch>, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();
    log::info!("Searching for stocks");

    let stocks =
        if search.name.is_some() || search.address.is_some() {
            let mut query = "SELECT * FROM branches_view WHERE".to_string();
            let mut count = 1;

            if let Some(name) = &search.name {
                query.push_str(&format!(" name = '{}'", name));
                count += 1;
            }

            if let Some(address) = &search.address {
                if count > 1 {
                    query.push_str(" AND");
                }
                query.push_str(&format!(" address = '{}'", address));
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
                    .query("SELECT * FROM branches_view;", &[])
                    .map(|rows: Vec<Row>| rows_to_stocks(rows))
            })
                .await?
        };
    match stocks {
        Ok(stocks) => Ok(stocks),
        Err(err) => Err(Box::new(err)),
    }
}