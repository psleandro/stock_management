use std::path::PathBuf;
use std::error::Error;

use calamine::{open_workbook, Xlsx, Reader, RangeDeserializerBuilder};
use serde::Deserialize;

use crate::infra::models::NewProductRow;

#[derive(Deserialize)]
struct ProductRecord {
    name: String,
    brand: Option<String>,
    unity: Option<String>,
    min_stock: Option<i32>,
    observation: Option<String>,
}

pub fn import_products(path: PathBuf) -> Result<Vec<NewProductRow>, Box<dyn Error>> {

    let mut workbook: Xlsx<_> = open_workbook(path).unwrap();

    let range = workbook.worksheet_range("Products")?;
    
    let headers = &["name", "brand", "unity", "min_stock", "observation"];

    let iter_records = RangeDeserializerBuilder::with_headers(headers).from_range(&range)?;
    
    let valid_records = iter_records
        .filter_map(|result: Result<ProductRecord, calamine::DeError>| {
            match result {
                Ok(record) => {
                    Some(NewProductRow {
                        name: record.name,
                        unity: record.unity,
                        brand: record.brand,
                        min_stock: record.min_stock,
                        observation: record.observation,
                    })
                },
                _ => None
            }
        }).collect();

    Ok(valid_records)
}