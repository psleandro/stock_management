use std::{io::Error, path::PathBuf};

use rust_xlsxwriter::*;

use crate::domain::place::Place;

pub fn export_places(places: &[Place], path_to_save: PathBuf) -> Result<bool, Error> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let column_headers = [
        "id",
        "name",
        "created_at",
        "updated_at",
    ];

    let _ = worksheet.set_name("Places");

    for (column_number, header_name) in column_headers.iter().enumerate() {
        let _ = worksheet.write(0, column_number as u16, *header_name);
    }

    for (index, place) in places.iter().enumerate() {
        let row = (index + 1) as u32;

        let values = vec![
            place.id.to_string(),
            place.name.clone(),
            place.created_at.to_string(),
            place.updated_at.to_string(),
        ];

        for (column, value) in values.iter().enumerate() {
            let _ = worksheet.write(row, column as u16, value);
        }

    }

    let _ = workbook.save(path_to_save);

    Ok(true)
}