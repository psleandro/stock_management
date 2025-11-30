use eframe::egui;
use egui::{ComboBox, Id, Modal, RichText, Sides};
use validator::{Validate};

use crate::infra::db;
use crate::infra::repositories::product_repository;
use crate::infra::models::{NewProductRow, EditProductRow};

use crate::domain::product::Product;

const DEFAULT_SPACING: f32 = 16.0;
const FORM_SPACING: f32 = DEFAULT_SPACING / 2.0;

#[derive(Debug, Default)]
pub struct FormErrors {
    pub name: Option<String>,
    pub min_stock: Option<String>,
}

#[derive(Validate)]
pub struct ProductForm { 
    #[validate(length(min = 2, message = "Name must contain at least two characters"))]
    name: String,
    
    unity: &'static str,

    brand: String,
    
    #[validate(range(min = 0))]
    min_stock: i32,
    observation: String,
}
#[derive(Debug)]
pub struct ProductFormModal {
	should_close: bool,

    id: Option<i32>,
    name: String,
    brand: String,
    unity: &'static str,
    min_stock: String,
    observation: String,
    errors: FormErrors,
}


impl ProductFormModal {

	pub fn new(product: Option<&Product>) -> Self {
        let errors = FormErrors::default();
        let should_close = false;

        match product {
            Some(prod) => Self { 
                should_close,
                errors,
                id: Some(prod.id),
                name: prod.name.clone(),
                brand: prod.brand.clone().unwrap_or_default(),
                unity: "un",
                min_stock: prod.min_stock.to_string(),
                observation: prod.observation.clone().unwrap_or_default(),
            },
            None => Self { 
                should_close,
                errors,
                id: None,
                name: "".to_owned(),
                brand: "".to_owned(),
                unity: "un",
                min_stock: "".to_owned(),
                observation: "".to_owned(),
            }
        }
	}

	pub fn show(&mut self, ui: &mut egui::Ui) -> (bool, Option<Product>) {
        let mut created_product = None;

 	 	let modal = Modal::new(Id::new("New Product")).show(ui.ctx(), |ui| {
		  ui.heading( if self.id.is_none() { "New Product" } else { "Edit Product" });

                ui.separator();
                ui.add_space(DEFAULT_SPACING / 2.0);

                ui.label("Name: ");
                ui.text_edit_singleline(&mut self.name);
                if let Some(error) = &self.errors.name {
                    ui.label(
                        RichText::new(error).color(ui.visuals().error_fg_color)
                    );
                }

                ui.add_space(FORM_SPACING);

                ui.label("Brand");
                ui.text_edit_singleline(&mut self.brand);

                ui.add_space(FORM_SPACING);

                ui.horizontal(|ui| {
                    ui.label("Unity");
                    
                    ComboBox::new("unity", "")
                    .selected_text(self.unity)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.unity, "un", "un");
                    });
                });

                ui.add_space(FORM_SPACING);

                ui.label("Min Stock");
                ui.text_edit_singleline(&mut self.min_stock);
                if let Some(error) = &self.errors.min_stock {
                    ui.label(
                        RichText::new(error).color(ui.visuals().error_fg_color)
                    );
                }

                ui.add_space(FORM_SPACING);

                ui.label("Observation");
                ui.text_edit_multiline(&mut self.observation);

                ui.add_space(DEFAULT_SPACING / 2.0);
                ui.separator();
                ui.add_space(FORM_SPACING);

                  Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.button("Save").clicked() {
                           if let Some(product) = self.validate_form() {
                                let mut connection = db::establish_connection();
                                
                                match self.id {
                                    Some(id,) => {
                                        let product = EditProductRow {
                                            id,
                                            name: product.name,
                                            unity: Some(product.unity.into()),
                                            brand: Some(product.brand),
                                            min_stock: Some(product.min_stock),
                                            observation: Some(product.observation),
                                        };

                                        if let Ok(updated) = product_repository::edit_product(&mut connection, product) {
                                            created_product = Some(updated);
                                        }
                                    }
                                    None => {
                                        let new_product = NewProductRow {
                                            name: product.name,
                                            unity: Some(product.unity.into()),
                                            brand: Some(product.brand),
                                            min_stock: Some(product.min_stock),
                                            observation: Some(product.observation),
                                        };

                                        if let Ok(created) = product_repository::create_product(&mut connection, new_product) {
                                            created_product = Some(created);
                                        }
                                    }
                                }
                                
                                self.should_close = true;
                           }
                        }

                        if ui.button("Cancel").clicked() {
                            self.should_close = true;
                        }
                    }
                );
		});

		if modal.should_close(){
			self.should_close = true;
		}

		(self.should_close, created_product)
  	}

    fn validate_form(&mut self) -> Option<ProductForm>{
        self.errors = FormErrors::default();

        let min_stock = match self.min_stock.parse::<i32>() {
            Ok(value) => value,
            Err(_) => {
                self.errors.min_stock = Some("Min stock should be a valide integer".into());
                -1
            }
        };

        let product_data = ProductForm {
            name: self.name.clone(),
            brand: self.brand.clone(),
            min_stock,
            unity: self.unity,
            observation: self.observation.clone()
        };


        match product_data.validate() {
            Ok(_) => Some(product_data),
            Err(error)=> {
                if let Some(name_error)= error.field_errors().get("name") {
                    self.errors.name = Some(name_error[0].clone().message.unwrap_or_default().to_string());
                }
                None
            }
        }
    }
}