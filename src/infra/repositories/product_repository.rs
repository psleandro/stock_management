use diesel::prelude::*;
use std::error::Error;

use crate::infra::models::{ProductRow, NewProductRow};
use crate::infra::schema::products;
use crate::domain::product::Product;

pub fn list_products(conn: &mut SqliteConnection) -> Result<Vec<Product>, Box<dyn Error>> {
  	let product_list: Vec<ProductRow> = products::table
        .load(conn)
        .expect("Error loading products");

  	let prods = product_list.into_iter()
    	.map(|product| product.try_into())
		.collect::<Result<Vec<_>, _>>()?;

    Ok(prods)
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