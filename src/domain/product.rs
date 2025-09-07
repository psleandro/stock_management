use chrono::NaiveDateTime;

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
