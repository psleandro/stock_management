use chrono::NaiveDateTime;

use crate::infra::models::{ProductRow};

const NAIVE_DATE_TIME_PATTERN: &str =  "%Y-%m-%d %H:%M:%S";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Product {
	pub id: i32,
	pub name: String,
	pub unity: Option<String>,
	pub brand: Option<String>,
	pub min_stock: i32,
	pub observation: Option<String>,
	pub created_at: NaiveDateTime,
	pub updated_at: NaiveDateTime,
	pub deleted_at: Option<NaiveDateTime>,
}

impl TryFrom<ProductRow> for Product {
    type Error = chrono::ParseError;

    fn try_from(row: ProductRow) -> Result<Self, Self::Error> {
        Ok(Product {
            id: row.id,
            name: row.name,
            unity: row.unity,
            brand: row.brand,
            min_stock: row.min_stock,
            observation: row.observation,
            created_at: NaiveDateTime::parse_from_str(&row.created_at, NAIVE_DATE_TIME_PATTERN)?,
            updated_at: NaiveDateTime::parse_from_str(&row.updated_at, NAIVE_DATE_TIME_PATTERN)?,
            deleted_at: match row.deleted_at {
                Some(val) => Some(NaiveDateTime::parse_from_str(&val, NAIVE_DATE_TIME_PATTERN)?),
                None => None,
            },
        })
    }
}