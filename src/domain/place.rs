use chrono::NaiveDateTime;

use crate::infra::models::PlaceRow;

const NAIVE_DATE_TIME_PATTERN: &str =  "%Y-%m-%d %H:%M:%S";

#[derive(Debug, Clone)]
pub struct Place {
  	pub id: i32,
  	pub name: String,
 	pub created_at: NaiveDateTime,
  	pub updated_at: NaiveDateTime,
  	pub deleted_at: Option<NaiveDateTime>,
}

impl TryFrom<PlaceRow> for Place {
    type Error = chrono::ParseError;

    fn try_from(row: PlaceRow) -> Result<Self, Self::Error> {
        Ok(Place {
            id: row.id,
            name: row.name,
            created_at: NaiveDateTime::parse_from_str(&row.created_at, NAIVE_DATE_TIME_PATTERN)?,
            updated_at: NaiveDateTime::parse_from_str(&row.updated_at, NAIVE_DATE_TIME_PATTERN)?,
            deleted_at: match row.deleted_at {
                Some(val) => Some(NaiveDateTime::parse_from_str(&val, NAIVE_DATE_TIME_PATTERN)?),
                None => None,
            },
        })
    }
}