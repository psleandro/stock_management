use iced::{Alignment, Border, Color, Length};
use iced::widget::{button, Button, Container, Column, Text, Theme};
use iced::border::Radius;


use crate::{ScreenId, Message};

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

	pub fn view(&self, &screen: &ScreenId) -> Column<'static, Message> {
		 let side_menu = self.menus.iter().fold(
            Column::new().width(Length::Fill),
            |column, menu| {
                let mut menu_item = Button::new(Text::new(menu.0))
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

                if screen != menu.1 {
                    menu_item = menu_item.on_press(Message::SwitchScreen(menu.1));
                }

                column.push(menu_item)
            }
        );

        let side_header = Container::new(Text::new("Stock Management"))
            .width(Length::Fill).height(80)
            .align_y(Alignment::Center)
            .align_x(Alignment::Center);
        
		let sidebar = Column::new()
			.width(Length::Fixed(200.0))
            .push(side_header)
            .push(side_menu);

        sidebar
	}
}