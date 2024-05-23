use chrono::NaiveDateTime;
use postgres_from_row::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Stock {
    pub id: i32,
    pub product_id: i32,
    pub product_name: String,
    pub branch_id: i32,
    pub branch_name: String,
    pub quantity: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Default, Deserialize)]
pub struct Search {
    pub product_name: Option<String>,
    pub branch_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct StockRequest {
    pub product_id: i32,
    pub branch_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct StockUpdate {
    pub quantity: i32,
}
