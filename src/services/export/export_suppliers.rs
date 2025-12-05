use std::{io::Error, path::PathBuf};

use rust_xlsxwriter::*;

use crate::domain::supplier::Supplier;

pub fn export_suppliers(suppliers: &[Supplier], path_to_save: PathBuf) -> Result<bool, Error> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let column_headers = [
        "id",
        "name",
        "created_at",
        "updated_at",
    ];

    let _ = worksheet.set_name("Suppliers");

    for (column_number, header_name) in column_headers.iter().enumerate() {
        let _ = worksheet.write(0, column_number as u16, *header_name);
    }

    for (index, supplier) in suppliers.iter().enumerate() {
        let row = (index + 1) as u32;

        let values = vec![
            supplier.id.to_string(),
            supplier.name.clone(),
            supplier.created_at.to_string(),
            supplier.updated_at.to_string(),
        ];

        for (column, value) in values.iter().enumerate() {
            let _ = worksheet.write(row, column as u16, value);
        }

    }

    let _ = workbook.save(path_to_save);

    Ok(true)
}