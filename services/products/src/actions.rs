use std::error::Error;

use database::PgPool;

use crate::models::{Product, Search};

pub async fn get_all_products(pool: &PgPool) -> Result<Vec<Product>, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();

    let products = conn
        .interact(|client| {
            client
                .query("SELECT * FROM products_view;", &[])
                .map(|rows| {
                    rows.iter()
                        .map(|row| Product {
                            id: row.get(0),
                            name: row.get(1),
                            description: row.get(2),
                            price: row.get(3),
                        })
                        .collect()
                })
        })
        .await?;
    match products {
        Ok(products) => Ok(products),
        Err(err) => Err(Box::new(err)),
    }
}

pub async fn search_products(
    pool: &PgPool,
    search: Search,
) -> Result<Vec<Product>, Box<dyn Error>> {
    let conn = pool.get().await.unwrap();

    let name = search.name.unwrap_or("".to_string());
    let min_price = search.min_price.unwrap_or(0.into());
    let max_price = search.max_price.unwrap_or(100000.into());
    let limit = search
        .limit
        .unwrap_or(10)
        .to_string()
        .parse::<i32>()
        .unwrap();
    let offset = search
        .offset
        .unwrap_or(0)
        .to_string()
        .parse::<i32>()
        .unwrap();

    let products;

    if search.branch_id.clone().is_none() {
        products = conn
            .interact(move |client| {
                client
                    .query(
                        "SELECT * FROM fn_products_view_filters_name_price($1, $2, $3, $4, $5);",
                        &[&name, &min_price, &max_price, &limit, &offset],
                    )
                    .map(|rows| {
                        rows.iter()
                            .map(|row| Product {
                                id: row.get(0),
                                name: row.get(1),
                                description: row.get(2),
                                price: row.get(3),
                            })
                            .collect()
                    })
            })
            .await?;
    } else {
        products = conn.interact(move |client| {
            client.query("SELECT * FROM fn_products_view_filters_name_price_branch($1, $2, $3, $4, $5, $6);", &[&name, &min_price, &max_price, &search.branch_id.unwrap(), &limit, &offset])
                .map(|rows| {
                    rows.iter().map(|row| {
                        Product {
                            id: row.get(0),
                            name: row.get(1),
                            description: row.get(2),
                            price: row.get(3),
                        }
                    }).collect()
                })
        }).await?;
    }
    match products {
        Ok(products) => Ok(products),
        Err(err) => Err(Box::new(err)),
    }
}
