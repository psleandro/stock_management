use iced::{Background, Element, Length};
use iced::widget::{container, Container, Row, Text, Theme};

mod widgets;

use crate::widgets::sidebar::{SideBar};


#[derive(Default)]
struct StockManagement {
    screen: Screen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Dashboard,
    Products,
    Suppliers,
    Places,
    InventoryTransactions,
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Dashboard
    }
}

#[derive(Debug, Clone)]
enum Message {
    SwitchScreen(Screen),
}

impl StockManagement {
    fn update(&mut self, message: Message) {
        match message {
            Message::SwitchScreen(screen) => self.screen = screen,
        }
    }

    fn view(&self) -> Element<Message> {
        
        let sidebar = SideBar::new().view(&self.screen);

        let divider = Container::new("")
            .width(Length::Fixed(1.0)).height(Length::Fill)
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();
                let color = palette.background.weak.color;

                container::Style{
                    background: Some(Background::Color(color)),
                    ..Default::default()
                }
            });

        let content: Text<_> = match self.screen {
            Screen::Dashboard => Text::new("Dashboard"),
            Screen::Products => Text::new("Products"),
            Screen::Suppliers => Text::new("Suppliers"),
            Screen::Places => Text::new("Places"),
            Screen::InventoryTransactions => Text::new("Transactions"),
        };

        Row::new()
            .push(sidebar)
            .push(divider)
            .push(content)
            .into()
    }
}


fn main() -> iced::Result {
    iced::run(
        "Stock Management",
        StockManagement::update,
        StockManagement::view
    )
}
