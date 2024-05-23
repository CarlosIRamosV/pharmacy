use postgres::Row;
use postgres_from_row::FromRow;

use crate::models::Branch;

pub fn rows_to_stocks(rows: Vec<Row>) -> Vec<Branch> {
    rows.iter().map(|row| Branch::from_row(&row)).collect()
}
