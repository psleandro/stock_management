use iced::border::Radius;
use iced::{Background, Border, Color, Element, Length, Alignment};
use iced::widget::{button, container, Button, Column, Container, Row, Text, Theme};

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
        
        let menus: [(&'static str, Screen); 5]  = [
            ("Dashboard", Screen::Dashboard),
            ("Products", Screen::Products),
            ("Suppliers", Screen::Suppliers),
            ("Places", Screen::Places),
            ("Inventory Transactions", Screen::InventoryTransactions),
        ];
            
            
        let side_menu = menus.iter().fold(
            Column::new().width(Length::Fill),
            |column, menu| {
                let mut button = Button::new(Text::new(menu.0))
                    .width(Length::Fill)
                    .style(|theme: &Theme, status| {
                        let palette = theme.extended_palette();
                        
                        match status {
                            button::Status::Active => {
                                button::Style{
                                    text_color: palette.background.base.text,
                                    border: Border{ radius: Radius::new(0), width: 0.0, color: Color::BLACK},
                                    ..Default::default()
                                }
                            }
                            _ => button::primary(theme, status)
                        }
                    });

                if self.screen != menu.1 {
                    button = button.on_press(Message::SwitchScreen(menu.1));
                }

                column.push(button)
            }
        );

        let side_header = Container::new(Text::new("Stock Management"))
            .width(Length::Fill).height(80)
            .align_y(Alignment::Center)
            .align_x(Alignment::Center);
        

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

        let sidebar = Column::new().width(Length::Fixed(200.0))
            .push(side_header)
            .push(side_menu);

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
