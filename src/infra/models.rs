use diesel::prelude::*;
use crate::infra::schema::{places, products, suppliers};


#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name=products)]
pub struct ProductRow {
	pub id: i32,
	pub name: String,
	pub unity: Option<String>,
	pub brand: Option<String>,
	pub min_stock: i32,
	pub observation: Option<String>,
	pub created_at: String,
	pub updated_at: String,
	pub deleted_at: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name=products)]
pub struct NewProductRow {
	pub name: String,
	pub unity: Option<String>,
	pub brand: Option<String>,
	pub min_stock: Option<i32>,
	pub observation: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name=products)]
pub struct EditProductRow {
	pub id: i32,
	pub name: String,
	pub unity: Option<String>,
	pub brand: Option<String>,
	pub min_stock: Option<i32>,
	pub observation: Option<String>,
}


#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name=suppliers)]
pub struct SupplierRow {
	pub id: i32,
	pub name: String,
	pub created_at: String,
	pub updated_at: String,
	pub deleted_at: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name=suppliers)]
pub struct NewSupplierRow {
	pub name: String,
}

#[derive(AsChangeset)]
#[diesel(table_name=suppliers)]
pub struct EditSupplierRow {
	pub id: i32,
	pub name: String,
}


#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name=places)]
pub struct PlaceRow {
	pub id: i32,
	pub name: String,
	pub created_at: String,
	pub updated_at: String,
	pub deleted_at: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name=places)]
pub struct NewPlaceRow {
	pub name: String,
}

#[derive(AsChangeset)]
#[diesel(table_name=places)]
pub struct EditPlaceRow {
	pub id: i32,
	pub name: String,
}