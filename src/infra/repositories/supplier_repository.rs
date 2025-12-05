use diesel::prelude::*;
use std::error::Error;

use crate::infra::models::{SupplierRow, NewSupplierRow, EditSupplierRow};
use crate::infra::schema::suppliers;
use crate::domain::supplier::Supplier;

use chrono::Utc;

const NAIVE_DATE_TIME_PATTERN: &str =  "%Y-%m-%d %H:%M:%S";

pub fn list_suppliers(conn: &mut SqliteConnection, search: &str) -> Result<Vec<Supplier>, Box<dyn Error>> {
    let search_like = format!("%{}%", search);

  	let mut suppliers_query= suppliers::table.filter(suppliers::deleted_at.is_null()).into_boxed();

    let filter_expression =  suppliers::name.like(&search_like);

    if let Ok(search_number) = search.parse::<i32>(){
        suppliers_query = suppliers_query.filter(
            filter_expression
                .or(suppliers::id.eq(search_number))
        );
    } else {
        suppliers_query = suppliers_query.filter(filter_expression);
    }
        
    let supplier_list: Vec<SupplierRow>  = suppliers_query.load(conn).expect("Error loading suppliers");

  	let prods = supplier_list.into_iter()
    	.map(|supplier| supplier.try_into())
		.collect::<Result<Vec<_>, _>>()?;

    Ok(prods)
}

pub fn create_supplier(conn: &mut SqliteConnection, new_supplier: NewSupplierRow) -> Result<Supplier, Box<dyn Error>> {
    diesel::insert_into(suppliers::table)
        .values(&new_supplier)
        .execute(conn)
        .expect("Failed to insert supplier");

    let created_supplier = suppliers::table
        .order(suppliers::id.desc())
        .first::<SupplierRow>(conn)
        .expect("Failed to retrieve created supplier");

    let supplier_item = created_supplier.try_into()?;

    Ok(supplier_item)
}

pub fn create_suppliers(conn: &mut SqliteConnection, new_suppliers: &[NewSupplierRow]) -> Result<Vec<Supplier>, Box<dyn Error>> {
    diesel::insert_into(suppliers::table)
        .values(new_suppliers)
        .execute(conn)
        .expect("Failed to insert suppliers");

    let created_suppliers: Vec<SupplierRow> = suppliers::table
        .order(suppliers::id.desc())
        .limit(new_suppliers.len() as i64)
        .load(conn)
        .expect("Failed to retrieve created suppliers");

    let new_suppliers = created_suppliers.into_iter()
        .rev()
        .map(|p| p.try_into())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(new_suppliers)
}

pub fn edit_supplier(conn: &mut SqliteConnection, supplier: EditSupplierRow) -> Result<Supplier, Box<dyn Error>> {
    let supplier_id = supplier.id;

    diesel::update(suppliers::table.find(supplier_id))
        .set((
            &supplier,
            suppliers::updated_at.eq(Utc::now().format(NAIVE_DATE_TIME_PATTERN).to_string())
        ))
        .execute(conn)
        .expect("Failed to update supplier");

    
    let updated_supplier = suppliers::table
        .filter(suppliers::id.eq(supplier_id))
        .first::<SupplierRow>(conn)
        .expect("Failed to retrieve updated supplier");

    let supplier_item = updated_supplier.try_into()?;

    Ok(supplier_item)
}

pub fn delete_supplier(conn: &mut SqliteConnection, supplier_id: i32) -> Result<bool, Box<dyn Error>> {
    let deleted = diesel::update(suppliers::table.find(supplier_id))
        .set(suppliers::deleted_at.eq(Some(Utc::now().naive_utc().format(NAIVE_DATE_TIME_PATTERN).to_string())))
        .execute(conn)?;

    Ok(deleted > 0)
}