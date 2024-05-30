use postgres::Row;

use crate::models::Product;

pub fn rows_to_products(rows: Vec<Row>) -> Vec<Product> {
    rows.iter().map(|row| Product::from_row(&row)).collect()
}
