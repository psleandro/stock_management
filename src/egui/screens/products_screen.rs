use eframe::egui;
use egui::{Direction, Layout};
use egui_extras::{Size, StripBuilder, Column, TableBuilder};

use crate::infra::db;
use crate::infra::repositories::product_repository;
use crate::infra::models::NewProductRow;
use crate::domain::product::Product;

const DEFAULT_SPACING: f32 = 16.0;
const ITEM_HEIGHT: f32 = 24.0;

pub struct ProductsScreen {
    pub products: Vec<Product>,
}

impl ProductsScreen {
    pub fn new() -> Self {
        let mut connection = db::establish_connection();
        let products = product_repository::list_products(&mut connection).unwrap_or_default();
        Self { products }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let add_product_btn = egui::Button::new(
            egui::RichText::new("Add Product").color(egui::Color32::WHITE)
        ).fill(ui.visuals().selection.bg_fill);

        ui.horizontal(|ui| {
            ui.heading("Products");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.add(add_product_btn).clicked() {
                let mut connection = db::establish_connection();
                let new_product = NewProductRow {
                    name: format!("Product Large Name Testing It To fill all space, is it possible? {}", self.products.len() + 1),
                    unity: Some("un".into()),
                    brand: Some("Brand X".into()),
                    min_stock: Some(0),
                    observation: None,
                };
                if let Ok(created) = product_repository::create_product(&mut connection, new_product) {
                    self.products.push(created);
                }
            }
            });
        });

        ui.add_space(DEFAULT_SPACING);

        StripBuilder::new(ui)
            .size(Size::remainder().at_least(100.0))
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        self.products_table(ui);
                    });
                });
            });
    }

    fn products_table(&mut self, ui: &mut egui::Ui){
        let mut to_delete: Option<i32> = None;

        let available_height = ui.available_height();

        TableBuilder::new(ui)
            .striped(true)
            .resizable(false) 
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .min_scrolled_height(200.0) 
            .column(Column::exact(64.0))      
            .column(
                Column::remainder()
                    .at_least(120.0)
                    .clip(true)
                    .resizable(false),
            )                                  
            .column(Column::exact(88.0))      
            .column(Column::exact(88.0))        
            .column(
                Column::auto()
                    .at_least(60.0)
                    .at_most(240.0)
                    .clip(true)
            )    
            .column(Column::exact(100.0))     
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height)      
            .header(ITEM_HEIGHT, |mut header| {
                header.col(|ui| { ui.heading("ID"); });
                header.col(|ui| { ui.heading("Name"); });
                header.col(|ui| {
                    ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                        ui.heading("Unity");
                    });
                });
                header.col(|ui| { ui.heading("Min Stock"); });
                header.col(|ui| { ui.heading("Observation"); });
                header.col(|ui| { 
                    ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                        ui.heading("Actions");
                    });
                });
            })
            .body(|mut body| {
                for product in &self.products {
                    body.row(ITEM_HEIGHT, |mut row| {
                        row.col(|ui| { ui.label(product.id.to_string()); });
                        row.col(|ui| { ui.label(&product.name); });
                        row.col(|ui| {
                            ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                ui.label(product.unity.clone().unwrap_or_default());
                            });
                        });
                        row.col(|ui| { ui.label(product.min_stock.to_string()); });
                        row.col(|ui| { ui.label(product.observation.clone().unwrap_or_default()); });
                        row.col(|ui| {
                            let delete_button = egui::Button::new(
                                egui::RichText::new("Delete").color(egui::Color32::WHITE)
                            ).fill(ui.visuals().error_fg_color);

                            if ui.add(delete_button).clicked() {
                                to_delete = Some(product.id);
                            }
                        });
                    });
                }

                if let Some(id) = to_delete {
                    let mut connection = db::establish_connection();
                    if product_repository::delete_product(&mut connection, id).is_ok() {
                        if let Some(pos) = self.products.iter().position(|p| p.id == id) {
                            self.products.remove(pos);
                        }
                    }
                }
            });
    }
}
