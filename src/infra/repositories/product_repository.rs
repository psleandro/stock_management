use diesel::prelude::*;
use std::error::Error;

use crate::infra::models::{ProductRow, NewProductRow, EditProductRow};
use crate::infra::schema::products;
use crate::domain::product::Product;

use chrono::Utc;

const NAIVE_DATE_TIME_PATTERN: &str =  "%Y-%m-%d %H:%M:%S";

pub fn list_products(conn: &mut SqliteConnection) -> Result<Vec<Product>, Box<dyn Error>> {
  	let product_list: Vec<ProductRow> = products::table
        .filter(products::deleted_at.is_null())
        .load(conn)
        .expect("Error loading products");

  	let prods = product_list.into_iter()
    	.map(|product| product.try_into())
		.collect::<Result<Vec<_>, _>>()?;

    Ok(prods)
}

pub fn get_product_by_id(conn: &mut SqliteConnection, product_id: i32) -> Result<Product, Box<dyn Error>> {
    let product = products::table
        .find(product_id)
        .first::<ProductRow>(conn)
        .expect("Failed to retrieve product");

    let product_item = product.try_into()?;

    Ok(product_item)
}

pub fn create_product(conn: &mut SqliteConnection, new_product: NewProductRow) -> Result<Product, Box<dyn Error>> {
    diesel::insert_into(products::table)
        .values(&new_product)
        .execute(conn)
        .expect("Failed to insert product");

    let created_product = products::table
        .order(products::id.desc())
        .first::<ProductRow>(conn)
        .expect("Failed to retrieve created product");

    let product_item = created_product.try_into()?;

    Ok(product_item)
}

pub fn create_products(conn: &mut SqliteConnection, new_products: &[NewProductRow]) -> Result<Vec<Product>, Box<dyn Error>> {
    diesel::insert_into(products::table)
        .values(new_products)
        .execute(conn)
        .expect("Failed to insert products");

    let created_products: Vec<ProductRow> = products::table
        .order(products::id.desc())
        .limit(new_products.len() as i64)
        .load(conn)
        .expect("Failed to retrieve created products");

    let new_products = created_products.into_iter()
        .rev()
        .map(|p| p.try_into())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(new_products)
}

pub fn edit_product(conn: &mut SqliteConnection, product: EditProductRow) -> Result<Product, Box<dyn Error>> {
    let product_id = product.id;

    diesel::update(products::table.find(product_id))
        .set((
            &product,
            products::updated_at.eq(Utc::now().format(NAIVE_DATE_TIME_PATTERN).to_string())
        ))
        .execute(conn)
        .expect("Failed to update product");

    
    let updated_product = products::table
        .filter(products::id.eq(product_id))
        .first::<ProductRow>(conn)
        .expect("Failed to retrieve updated product");

    let product_item = updated_product.try_into()?;

    Ok(product_item)
}

pub fn delete_product(conn: &mut SqliteConnection, product_id: i32) -> Result<bool, Box<dyn Error>> {
    let deleted = diesel::update(products::table.find(product_id))
        .set(products::deleted_at.eq(Some(Utc::now().naive_utc().format(NAIVE_DATE_TIME_PATTERN).to_string())))
        .execute(conn)?;

    Ok(deleted > 0)
}