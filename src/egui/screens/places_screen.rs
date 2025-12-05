use eframe::egui;
use egui::{Direction, Label, Layout, Sides};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use rfd::FileDialog;
use std::error::Error;

use crate::infra::db;
use crate::infra::repositories::place_repository;
use crate::domain::place::Place;
use crate::egui::components::modals::place_form_modal::PlaceFormModal;
use crate::services::export::export_places::export_places;
use crate::services::import::import_places::import_places;

const DEFAULT_SPACING: f32 = 16.0;
const ITEM_HEIGHT: f32 = 24.0;

pub struct PlacesScreen {
    pub places: Vec<Place>,
    pub place_form_modal: Option<PlaceFormModal>,
    pub place_to_delete: Option<Place>,
    pub error: Option<Box<dyn Error>>,
    pub search: String,
}

impl PlacesScreen {
    pub fn new() -> Self {
        let places = PlacesScreen::get_places_list("");

        Self {
            places,
            place_form_modal: None,
            place_to_delete: None,
            error: None,
            search: String::new(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let add_place_btn = egui::Button::new(
            egui::RichText::new("Add Place").color(egui::Color32::WHITE)
        ).fill(ui.visuals().selection.bg_fill);

        ui.horizontal(|ui| {
            ui.heading("Places");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.add(add_place_btn).clicked() {
                    self.place_form_modal = Some(PlaceFormModal::new(None));
                }

                if ui.add(egui::Button::new("Export")).clicked() {
                    match FileDialog::new().set_file_name("places.xlsx").save_file() {
                        Some(path) => {
                            let _ = export_places(&self.places, path);
                        },
                        None => {}
                    };
                }

                if ui.add(egui::Button::new("Import")).clicked() {
                    if let Some(path) = FileDialog::new().pick_file() {
                        match import_places(path) {
                            Ok(new_places_row) => {
                                let mut conn = db::establish_connection();

                                let creation_result = place_repository::create_places(
                                    &mut conn,
                                    &new_places_row
                                );

                                if let Ok(mut created_places) = creation_result {
                                    self.places.append(&mut created_places);
                                }
                            },
                            Err(error) => {
                                self.error = Some(error);
                            }
                        }
                    }
                }
            
                if ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("Search for place...")).changed() {
                    let filtered_places = PlacesScreen::get_places_list(&self.search);
                    self.places = filtered_places;
                };
            });
        });

        ui.add_space(DEFAULT_SPACING);

        StripBuilder::new(ui)
            .size(Size::remainder().at_least(100.0))
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        self.places_table(ui);
                    });
                });
            });

        if let Some(modal) = self.place_form_modal.as_mut() {
            let (should_close, upserted_place) = modal.show(ui);

            if should_close {
                self.place_form_modal = None;

                if let Some(place) = upserted_place {
                    if let Some(existing_place) = self.places.iter_mut().find(|p| p.id == place.id) {
                        *existing_place = place;
                    } else {
                        self.places.push(place);
                    }
                }
            }
        }

        if self.place_to_delete.is_some() {
            self.show_confirm_delete_alert(ui);
        }

        if self.error.is_some() {
           self.show_error_message(ui);
        }
    }

    fn places_table(&mut self, ui: &mut egui::Ui) {
        let avaiable_height = ui.available_height();

        TableBuilder::new(ui)
            .striped(true)
            .resizable(false)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::exact(64.0))
            .column(
                Column::remainder()
                .at_least(120.0)
                .clip(true)
                .resizable(false)
            )
            .column(Column::exact(100.0))     
            .min_scrolled_height(0.0)
            .max_scroll_height(avaiable_height)
            .header(ITEM_HEIGHT,|mut header| {
                header.col(|ui| { ui.heading("ID"); });
                header.col(|ui| { ui.heading("Name"); });
                header.col(|ui| { 
                    ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                        ui.heading("Actions");
                    });
                });
            })
            .body(|body| {
                body.rows(
                    ITEM_HEIGHT,
                    self.places.len(),
                    |mut row| {
                        let place_index = row.index();
                        let place = &self.places[place_index];


                        row.col(|ui| { ui.label(place.id.to_string()); });
                        row.col(|ui| { ui.label(place.name.clone()); });
                        row.col(|ui| {
                            let delete_button = egui::Button::new(
                            egui::RichText::new("Delete").color(egui::Color32::WHITE)
                            ).fill(ui.visuals().error_fg_color);

                             if ui.add(delete_button).clicked() {
                                self.place_to_delete = Some(place.clone());
                            }

                            if ui.add(egui::Button::new("Edit")).clicked() {
                                self.place_form_modal = Some(PlaceFormModal::new(Some(&place)));
                            }
                        });
                    }
                );
            });
    }

    fn show_confirm_delete_alert(&mut self, ui: &mut egui::Ui) {
        let alert = egui::Modal::new(egui::Id::new("Delete Place"))
            .show(ui.ctx(), |ui| {
                ui.heading("Delete Place");

                ui.separator();
                ui.add_space(DEFAULT_SPACING / 2.0);

                ui.add(
                Label::new(
                    format!("Are you sure you want to delete place '{}'?",
                            self.place_to_delete.as_ref().unwrap().name)
                        )
                );

                ui.add_space(DEFAULT_SPACING * 2.0);
                ui.separator();
                
                Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.add(egui::Button::new("Confirm")).clicked() {
                           let mut connection = db::establish_connection();

                            if place_repository::delete_place(&mut connection, self.place_to_delete.as_ref().unwrap().id).is_ok() {
                                if let Some(pos) = self.places.iter().position(|p| p.id == self.place_to_delete.as_ref().unwrap().id) {
                                    self.places.remove(pos);
                                }
                            }
                           
                            self.place_to_delete = None;
                        }

                        if ui.add(egui::Button::new("Cancel")).clicked() {
                            self.place_to_delete = None;
                        }
                    }
                );

            });

        if alert.should_close() {
            self.place_to_delete = None;
        }
    }

    fn show_error_message(&mut self, ui: &mut egui::Ui){
        if let Some(error) = &self.error {
            let error_message = error.to_string();

            let error_alert = egui::Modal::new(egui::Id::new("Error Message"))
                .show(ui.ctx(), |ui| {
                    ui.heading("Error");

                    ui.separator();
                    ui.add_space(DEFAULT_SPACING / 2.0);

                    ui.label(error_message);

                    ui.add_space(DEFAULT_SPACING);
                    ui.separator();


                    Sides::new().show(
                        ui,
                        |_ui| {},
                        |ui| {
                            if ui.button("OK").clicked() {
                                self.error = None;
                            }
                        }
                    );
                });

            if error_alert.should_close() {
                self.error = None;
            }
        }
    }

    fn get_places_list(search: &str) -> Vec<Place> {
        let mut connection = db::establish_connection();

        let places = place_repository::list_places(&mut connection, search)
            .unwrap_or_default();

        places
    }
}