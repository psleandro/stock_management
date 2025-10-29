use eframe::egui;
use egui::{ComboBox, Id, Modal, Sides};

use crate::infra::db;
use crate::infra::repositories::product_repository;
use crate::infra::models::NewProductRow;

use crate::domain::product::Product;

const DEFAULT_SPACING: f32 = 16.0;
const FORM_SPACING: f32 = DEFAULT_SPACING / 2.0;

pub struct ProductFormModal {
	should_close: bool,

    name: String,
    unity: &'static str,
    min_stock: String,
    observation: String,
}

impl ProductFormModal {

	pub fn new() -> Self {
		Self { 
            should_close: false,
            name: "".to_owned(),
            unity: "un",
            min_stock: "".to_owned(),
            observation: "".to_owned(),
        }
	}

	pub fn show(&mut self, ui: &mut egui::Ui) -> (bool, Option<Product>) {
        let mut created_product = None;

        let Self {
            should_close,
            name,
            unity,
            min_stock,
            observation,
        } = self;

 	 	let modal = Modal::new(Id::new("New Product")).show(ui.ctx(), |ui| {
		  ui.heading("New Product");

                ui.separator();
                ui.add_space(DEFAULT_SPACING / 2.0);

                ui.label("Name: ");
                ui.text_edit_singleline(name);

                ui.add_space(FORM_SPACING);

                ui.horizontal(|ui| {
                    ui.label("Unity");
                    
                    ComboBox::new("unity", "")
                    .selected_text(*unity)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(unity, "un", "un");
                    });
                });

                ui.add_space(FORM_SPACING);

                ui.label("Min Stock");
                ui.text_edit_singleline(min_stock);

                ui.add_space(FORM_SPACING);

                ui.label("Observation");
                ui.text_edit_multiline(observation);

                ui.add_space(DEFAULT_SPACING / 2.0);
                ui.separator();
                ui.add_space(FORM_SPACING);

                Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.button("Save").clicked() {
                           let mut connection = db::establish_connection();
                            let new_product = NewProductRow {
                                name: name.clone(),
                                unity: Some(unity.to_string()),
                                brand: Some("Brand X".into()),
                                min_stock: Some(min_stock.parse::<i32>().unwrap()),
                                observation: Some(observation.clone()),
                            };

                            if let Ok(created) = product_repository::create_product(&mut connection, new_product) {
                                created_product = Some(created);
                            }
                            
                            *should_close = true;
                        }

                        if ui.button("Cancel").clicked() {
                            *should_close = true;
                        }
                    }
                );
		});

		if modal.should_close(){
			*should_close = true;
		}

		(*should_close, created_product)
  	}
}