use postgres::Row;
use postgres_from_row::FromRow;

use crate::models::Stock;

pub fn rows_to_stocks(rows: Vec<Row>) -> Vec<Stock> {
    rows.iter().map(|row| Stock::from_row(&row)).collect()
}
