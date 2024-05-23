use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    // The price is stored as a decimal in the database
    pub price: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct Search {
    pub name: Option<String>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
    pub branch_id: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
