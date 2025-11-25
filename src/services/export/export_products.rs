use std::{io::Error, path::PathBuf};

use rust_xlsxwriter::*;

use crate::domain::product::Product;

pub fn export_products(products: &[Product], path_to_save: PathBuf) -> Result<bool, Error> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let column_headers = [
        "id",
        "name",
        "brand",
        "unity",
        "min_stock",
        "observation",
        "created_at",
        "updated_at",
    ];

    let _ = worksheet.set_name("Products");

    for (column_number, header_name) in column_headers.iter().enumerate() {
        let _ = worksheet.write(0, column_number as u16, *header_name);
    }

    for (index, product) in products.iter().enumerate() {
        let row = (index + 1) as u32;

        let values = vec![
            product.id.to_string(),
            product.name.clone(),
            product.brand.clone().unwrap_or_default(),
            product.unity.clone().unwrap_or_default(),
            product.min_stock.to_string(),
            product.observation.clone().unwrap_or_default(),
            product.created_at.to_string(),
            product.updated_at.to_string(),
        ];

        for (column, value) in values.iter().enumerate() {
            let _ = worksheet.write(row, column as u16, value);
        }

    }

    let _ = workbook.save(path_to_save);

    Ok(true)
}