use std::path::PathBuf;
use std::error::Error;

use calamine::{open_workbook, Xlsx, Reader, RangeDeserializerBuilder};
use serde::Deserialize;

use crate::infra::models::NewSupplierRow;

#[derive(Deserialize)]
struct SupplierRecord {
    name: String,
}

pub fn import_suppliers(path: PathBuf) -> Result<Vec<NewSupplierRow>, Box<dyn Error>> {

    let mut workbook: Xlsx<_> = open_workbook(path).unwrap();

    let range = workbook.worksheet_range("Suppliers")?;
    
    let headers = &["name"];

    let iter_records = RangeDeserializerBuilder::with_headers(headers).from_range(&range)?;
    
    let valid_records = iter_records
        .filter_map(|result: Result<SupplierRecord, calamine::DeError>| {
            match result {
                Ok(record) => {
                    Some(NewSupplierRow { name: record.name })
                },
                _ => None
            }
        }).collect();

    Ok(valid_records)
}