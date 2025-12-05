use std::path::PathBuf;
use std::error::Error;

use calamine::{open_workbook, Xlsx, Reader, RangeDeserializerBuilder};
use serde::Deserialize;

use crate::infra::models::NewPlaceRow;

#[derive(Deserialize)]
struct PlaceRecord {
    name: String,
}

pub fn import_places(path: PathBuf) -> Result<Vec<NewPlaceRow>, Box<dyn Error>> {

    let mut workbook: Xlsx<_> = open_workbook(path).unwrap();

    let range = workbook.worksheet_range("Places")?;
    
    let headers = &["name"];

    let iter_records = RangeDeserializerBuilder::with_headers(headers).from_range(&range)?;
    
    let valid_records = iter_records
        .filter_map(|result: Result<PlaceRecord, calamine::DeError>| {
            match result {
                Ok(record) => {
                    Some(NewPlaceRow { name: record.name })
                },
                _ => None
            }
        }).collect();

    Ok(valid_records)
}