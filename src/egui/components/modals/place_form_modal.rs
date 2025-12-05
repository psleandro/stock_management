use egui::{Id, Modal, RichText, Sides};
use validator::Validate;

use crate::infra::db;
use crate::infra::repositories::place_repository;
use crate::infra::models::{NewPlaceRow, EditPlaceRow};

use crate::domain::place::Place;

const DEFAULT_SPACING: f32 = 16.0;
const FORM_SPACING: f32 = DEFAULT_SPACING / 2.0;

#[derive(Default)]
pub struct FormErrors {
    pub name: Option<String>,
}

#[derive(Validate)]
pub struct PlaceForm {
    #[validate(length(min = 2, message = "Name must contain at least two characters"))]
    name: String,
}

pub struct PlaceFormModal {
    should_close: bool,

    id: Option<i32>,
    name: String,

    errors: FormErrors,
}

impl PlaceFormModal {
    pub fn new (place: Option<&Place>) -> Self {
        let errors = FormErrors::default();
        let should_close = false;

        match place {
            Some(place) => Self {
                should_close,
                errors,
                id: Some(place.id),
                name: place.name.clone(),
                
            },
            None => Self {
                should_close,
                errors,
                id: None,
                name: String::new(),
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> (bool, Option<Place>) {
        let mut created_place = None;

        let modal = Modal::new(Id::new("New Place")).show(ui.ctx(), |ui| {
            ui.heading(if self.id.is_none() { "New Place" } else { "Edit Place" });
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
                        if let Some(place) = self.validate_form() {
                            let mut connection = db::establish_connection();
                                
                            match self.id {
                                Some(id,) => {
                                    let place = EditPlaceRow {
                                        id,
                                        name: place.name,
                                    };

                                    if let Ok(updated) = place_repository::edit_place(&mut connection, place) {
                                        created_place = Some(updated);
                                    }
                                }
                                None => {
                                    let new_place = NewPlaceRow {
                                        name: place.name,
                                    };

                                    if let Ok(created) = place_repository::create_place(&mut connection, new_place) {
                                        created_place = Some(created);
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

        (self.should_close, created_place)
    }

     fn validate_form(&mut self) -> Option<PlaceForm>{
        self.errors = FormErrors::default();

        let place_data = PlaceForm {
            name: self.name.clone(),
        };


        match place_data.validate() {
            Ok(_) => Some(place_data),
            Err(error)=> {
                if let Some(name_error)= error.field_errors().get("name") {
                    self.errors.name = Some(name_error[0].clone().message.unwrap_or_default().to_string());
                }
                None
            }
        }
    }
}