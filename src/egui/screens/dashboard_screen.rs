use eframe::egui;
use egui::{Direction, Label, Layout, Sides};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use rfd::FileDialog;
use std::error::Error;

use crate::infra::db;
use crate::infra::repositories::supplier_repository;
use crate::domain::supplier::Supplier;
use crate::egui::components::modals::supplier_form_modal::SupplierFormModal;
use crate::services::export::export_suppliers::export_suppliers;
use crate::services::import::import_suppliers::import_suppliers;

const DEFAULT_SPACING: f32 = 16.0;
const ITEM_HEIGHT: f32 = 24.0;

pub struct DashboardScreen {
    pub entry_form_modal: Option<SupplierFormModal>,
    pub error: Option<Box<dyn Error>>,
}

impl DashboardScreen {
    pub fn new() -> Self {
        let suppliers = DashboardScreen::get_suppliers_list("");

        Self {
            entry_form_modal: None,
            error: None,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let add_supplier_btn = egui::Button::new(
            egui::RichText::new("Add Supplier").color(egui::Color32::WHITE)
        ).fill(ui.visuals().selection.bg_fill);

        ui.horizontal(|ui| {
            ui.heading("Dashboard");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // if ui.add(add_supplier_btn).clicked() {
                //     self.supplier_form_modal = Some(SupplierFormModal::new(None));
                // }

                // if ui.add(egui::Button::new("Export")).clicked() {
                //     match FileDialog::new().set_file_name("suppliers.xlsx").save_file() {
                //         Some(path) => {
                //             let _ = export_suppliers(&self.suppliers, path);
                //         },
                //         None => {}
                //     };
                // }

                // if ui.add(egui::Button::new("Import")).clicked() {
                //     if let Some(path) = FileDialog::new().pick_file() {
                //         match import_suppliers(path) {
                //             Ok(new_suppliers_row) => {
                //                 let mut conn = db::establish_connection();

                //                 let creation_result = supplier_repository::create_suppliers(
                //                     &mut conn,
                //                     &new_suppliers_row
                //                 );

                //                 if let Ok(mut created_suppliers) = creation_result {
                //                     self.suppliers.append(&mut created_suppliers);
                //                 }
                //             },
                //             Err(error) => {
                //                 self.error = Some(error);
                //             }
                //         }
                //     }
                // }
            
                // if ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("Search for supplier...")).changed() {
                //     let filtered_suppliers = SuppliersScreen::get_suppliers_list(&self.search);
                //     self.suppliers = filtered_suppliers;
                // };
            });
        });

        ui.add_space(DEFAULT_SPACING);

        StripBuilder::new(ui)
            .size(Size::remainder().at_least(100.0))
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        self.suppliers_table(ui);
                    });
                });
            });

        // if let Some(modal) = self.supplier_form_modal.as_mut() {
        //     let (should_close, upserted_supplier) = modal.show(ui);

        //     if should_close {
        //         self.supplier_form_modal = None;

        //         if let Some(supplier) = upserted_supplier {
        //             if let Some(existing_supplier) = self.suppliers.iter_mut().find(|p| p.id == supplier.id) {
        //                 *existing_supplier = supplier;
        //             } else {
        //                 self.suppliers.push(supplier);
        //             }
        //         }
        //     }
        // }

        // if self.supplier_to_delete.is_some() {
        //     self.show_confirm_delete_alert(ui);
        // }

        // if self.error.is_some() {
        //    self.show_error_message(ui);
        // }
    }

    fn suppliers_table(&mut self, ui: &mut egui::Ui) {
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
                    // self.suppliers.len(),
                    0,
                    |mut row| {
                        // let supplier_index = row.index();
                        // let supplier = &self.suppliers[supplier_index];


                        // row.col(|ui| { ui.label(supplier.id.to_string()); });
                        // row.col(|ui| { ui.label(supplier.name.clone()); });
                        // row.col(|ui| {
                        //     let delete_button = egui::Button::new(
                        //     egui::RichText::new("Delete").color(egui::Color32::WHITE)
                        //     ).fill(ui.visuals().error_fg_color);

                        //      if ui.add(delete_button).clicked() {
                        //         self.supplier_to_delete = Some(supplier.clone());
                        //     }

                        //     if ui.add(egui::Button::new("Edit")).clicked() {
                        //         self.supplier_form_modal = Some(SupplierFormModal::new(Some(&supplier)));
                        //     }
                        // });
                    }
                );
            });
    }

    fn show_confirm_delete_alert(&mut self, ui: &mut egui::Ui) {
        let alert = egui::Modal::new(egui::Id::new("Delete Supplier"))
            .show(ui.ctx(), |ui| {
                ui.heading("Delete Supplier");

                ui.separator();
                ui.add_space(DEFAULT_SPACING / 2.0);

                // ui.add(
                // Label::new(
                //     format!("Are you sure you want to delete supplier '{}'?",
                //             self.supplier_to_delete.as_ref().unwrap().name)
                //         )
                // );

                ui.add_space(DEFAULT_SPACING * 2.0);
                ui.separator();
                
                // Sides::new().show(
                //     ui,
                //     |_ui| {},
                //     |ui| {
                //         if ui.add(egui::Button::new("Confirm")).clicked() {
                //            let mut connection = db::establish_connection();

                //             if supplier_repository::delete_supplier(&mut connection, self.supplier_to_delete.as_ref().unwrap().id).is_ok() {
                //                 if let Some(pos) = self.suppliers.iter().position(|p| p.id == self.supplier_to_delete.as_ref().unwrap().id) {
                //                     self.suppliers.remove(pos);
                //                 }
                //             }
                           
                //             self.supplier_to_delete = None;
                //         }

                //         if ui.add(egui::Button::new("Cancel")).clicked() {
                //             self.supplier_to_delete = None;
                //         }
                //     }
                // );

            });

        // if alert.should_close() {
        //     self.supplier_to_delete = None;
        // }
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

    fn get_suppliers_list(search: &str) -> Vec<Supplier> {
        let mut connection = db::establish_connection();

        let suppliers = supplier_repository::list_suppliers(&mut connection, search)
            .unwrap_or_default();

        suppliers
    }
}