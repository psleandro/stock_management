pub mod components;
pub mod screens;
pub mod widgets;

use eframe::egui;
use crate::egui::widgets::sidebar::SideBar;
use crate::egui::screens::products_screen::ProductsScreen;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenId {
    Dashboard,
    Products,
    Suppliers,
    Places,
    InventoryTransactions,
}

pub struct StockManagement {
    pub active_screen: ScreenId,
    pub products_screen: Option<ProductsScreen>,
    pub sidebar: SideBar,
}

impl Default for StockManagement {
    fn default() -> Self {
        Self {
            active_screen: ScreenId::Dashboard,
            products_screen: Some(ProductsScreen::new()),
            sidebar: SideBar::new(),
        }
    }
}


impl eframe::App for StockManagement {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").resizable(false).show(ctx, |ui| {
            self.sidebar.ui(ui, &mut self.active_screen);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.active_screen {
                ScreenId::Dashboard => { ui.label("Dashboard content"); }
                ScreenId::Products => {
                    if let Some(screen) = &mut self.products_screen {
                        screen.ui(ui);
                    }
                }
                ScreenId::Suppliers => { ui.label("Suppliers content"); }
                ScreenId::Places => { ui.label("Places content"); }
                ScreenId::InventoryTransactions => { ui.label("Inventory content"); }
            }
        });
    }
}

pub fn run() {
    let app = StockManagement::default();
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Stock Management",
        native_options,
        Box::new(|_cc| Ok(Box::new(app) as Box<dyn eframe::App>)),
    )
    .unwrap();
}