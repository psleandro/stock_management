use eframe::egui;
use crate::egui::ScreenId;

pub struct SideBar {
    menus: [(&'static str, ScreenId); 5],
}

impl SideBar {
    pub fn new() -> Self {
        Self {
            menus: [
                ("Dashboard", ScreenId::Dashboard),
                ("Products", ScreenId::Products),
                ("Suppliers", ScreenId::Suppliers),
                ("Places", ScreenId::Places),
                ("Inventory Transactions", ScreenId::InventoryTransactions),
            ],
        }
    }

    pub fn ui(&self, ui: &mut egui::Ui, active_screen: &mut ScreenId) {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("Stock Management");
            ui.add_space(20.0);
        });

        for (label, screen) in &self.menus {
            let is_active = *active_screen == *screen;

            let button = egui::Button::new(*label)
                .fill(if is_active {
                    ui.visuals().selection.bg_fill
                } else {
                    ui.visuals().widgets.inactive.bg_fill
                })
                .min_size(egui::vec2(ui.available_width(), 30.0));

            if ui.add(button).clicked() && !is_active {
                *active_screen = *screen;
            }

            ui.add_space(5.0);
        }
    }
}
