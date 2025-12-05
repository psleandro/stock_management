use egui::{Id, Modal, RichText, Sides};
use validator::Validate;

use crate::infra::db;
use crate::infra::repositories::supplier_repository;
use crate::infra::models::{NewSupplierRow, EditSupplierRow};

use crate::domain::supplier::Supplier;

const DEFAULT_SPACING: f32 = 16.0;
const FORM_SPACING: f32 = DEFAULT_SPACING / 2.0;

#[derive(Default)]
pub struct FormErrors {
    pub name: Option<String>,
}

#[derive(Validate)]
pub struct SupplierForm {
    #[validate(length(min = 2, message = "Name must contain at least two characters"))]
    name: String,
}

pub struct SupplierFormModal {
    should_close: bool,

    id: Option<i32>,
    name: String,

    errors: FormErrors,
}

impl SupplierFormModal {
    pub fn new (supplier: Option<&Supplier>) -> Self {
        let errors = FormErrors::default();
        let should_close = false;

        match supplier {
            Some(supplier) => Self {
                should_close,
                errors,
                id: Some(supplier.id),
                name: supplier.name.clone(),
                
            },
            None => Self {
                should_close,
                errors,
                id: None,
                name: String::new(),
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> (bool, Option<Supplier>) {
        let mut created_supplier = None;

        let modal = Modal::new(Id::new("New Supplier")).show(ui.ctx(), |ui| {
            ui.heading(if self.id.is_none() { "New Supplier" } else { "Edit Supplier" });
            ui.separator();
            ui.add_space(DEFAULT_SPACING / 2.0);

            ui.label("Name: ");
            ui.text_edit_singleline(&mut self.name);
            if let Some(error) = &self.errors.name {
                ui.label(
                    RichText::new(error).color(ui.visuals().error_fg_color)
                );
            }

            ui.add_space(DEFAULT_SPACING / 2.0);
            ui.separator();
            ui.add_space(FORM_SPACING);


            Sides::new().show(
                ui,
                |_ui| {},
                |ui| {
                    if ui.button("Save").clicked() {
                        if let Some(supplier) = self.validate_form() {
                            let mut connection = db::establish_connection();
                                
                            match self.id {
                                Some(id,) => {
                                    let supplier = EditSupplierRow {
                                        id,
                                        name: supplier.name,
                                    };

                                    if let Ok(updated) = supplier_repository::edit_supplier(&mut connection, supplier) {
                                        created_supplier = Some(updated);
                                    }
                                }
                                None => {
                                    let new_supplier = NewSupplierRow {
                                        name: supplier.name,
                                    };

                                    if let Ok(created) = supplier_repository::create_supplier(&mut connection, new_supplier) {
                                        created_supplier = Some(created);
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

        (self.should_close, created_supplier)
    }

     fn validate_form(&mut self) -> Option<SupplierForm>{
        self.errors = FormErrors::default();

        let supplier_data = SupplierForm {
            name: self.name.clone(),
        };


        match supplier_data.validate() {
            Ok(_) => Some(supplier_data),
            Err(error)=> {
                if let Some(name_error)= error.field_errors().get("name") {
                    self.errors.name = Some(name_error[0].clone().message.unwrap_or_default().to_string());
                }
                None
            }
        }
    }
}