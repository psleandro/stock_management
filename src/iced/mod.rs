pub mod screens;
pub mod widgets;

use iced::{Background, Element, Length};
use iced::widget::{container, Container, Row, Text, Theme};

use crate::iced::screens::products_screen::{self, ProductsScreen};
use crate::iced::widgets::sidebar::SideBar;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenId {
    Dashboard,
    Products,
    Suppliers,
    Places,
    InventoryTransactions,
}

pub enum ActiveScreenInstance {
    Dashboard,
    Products(products_screen::ProductsScreen),
    Suppliers,
    Places,
    InventoryTransactions,
}

pub struct StockManagement {
    pub screen: ScreenId,
    pub active_screen: ActiveScreenInstance,
}

impl Default for StockManagement {
    fn default() -> Self {
        Self {
            screen: ScreenId::Dashboard,
            active_screen: ActiveScreenInstance::Dashboard,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SwitchScreen(ScreenId),
    Products(products_screen::ProductsScreenMessage),
}

impl StockManagement {
    pub fn update(&mut self, message: Message) {
        match (&mut self.active_screen, message) {
            (_, Message::SwitchScreen(screen)) => {
                self.screen = screen;
                self.active_screen = match screen {
                    ScreenId::Dashboard => ActiveScreenInstance::Dashboard,
                    ScreenId::Products => ActiveScreenInstance::Products(ProductsScreen::new()),
                    ScreenId::Suppliers => ActiveScreenInstance::Suppliers,
                    ScreenId::Places => ActiveScreenInstance::Places,
                    ScreenId::InventoryTransactions => ActiveScreenInstance::InventoryTransactions,
                }
            }
            (ActiveScreenInstance::Products(screen), Message::Products(message)) => {
                screen.update(message)
            }
            _ => {}
        }
    }

    pub fn view(&self) -> Element<Message> {
        let sidebar = SideBar::new().view(&self.screen);

        let divider = Container::new("")
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();
                let color = palette.background.weak.color;

                container::Style {
                    background: Some(Background::Color(color)),
                    ..Default::default()
                }
            });

        let content: Element<_> = match &self.active_screen {
            ActiveScreenInstance::Dashboard => Text::new("Dashboard").into(),
            ActiveScreenInstance::Products(screen) => screen.view().map(Message::Products),
            ActiveScreenInstance::Suppliers => Text::new("Suppliers").into(),
            ActiveScreenInstance::Places => Text::new("Places").into(),
            ActiveScreenInstance::InventoryTransactions => Text::new("Transactions").into(),
        };

        Row::new().push(sidebar).push(divider).push(content).into()
    }
}

pub fn run() -> iced::Result {
    iced::run(
        "Stock Management",
        StockManagement::update,
        StockManagement::view
    )
}
