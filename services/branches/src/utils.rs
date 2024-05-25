use postgres::Row;
use postgres_from_row::FromRow;

use crate::models::Branch;

pub fn rows_to_branches(rows: Vec<Row>) -> Vec<Branch> {
    rows.iter().map(|row| Branch::from_row(&row)).collect()
}
