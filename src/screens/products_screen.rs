use iced::{ Alignment, Element, Length };
use iced::widget::{Container, Column, Row, Text, Button, Space};

use crate::infra::db;
use crate::infra::repositories::product_repository;
use crate::infra::models::NewProductRow;

use crate::domain::product::Product;

const DEFAULT_SPACING: u16 = 24;

#[derive(Debug, Clone, PartialEq)]
pub struct ProductsScreen {
	products: Vec<Product>,
}

#[derive(Debug, Clone)]
pub enum ProductsScreenMessage {
	Create,
	Delete(i32),
}

impl ProductsScreen {
		pub fn new() -> Self {
			let mut connection = db::establish_connection();
			let products = product_repository::list_products(&mut connection).unwrap();
			Self { products }
	}

	pub fn update(&mut self, message: ProductsScreenMessage){
		match message {
			ProductsScreenMessage::Create => {
				let new_product = NewProductRow {
					name: format!("Product {}", self.products.len()),
					unity: Some("un".into()),
					brand: Some("Brand X".into()),
					min_stock: None,
					observation: None,
				};

				let mut connection = db::establish_connection();
				let product_created = product_repository::create_product(&mut connection, new_product);
				self.products.push(product_created.unwrap());
			},
			ProductsScreenMessage::Delete(product_id) => {
				let mut connection = db::establish_connection();
				product_repository::delete_product(&mut connection, product_id).unwrap();
				if let Some(pos) = self.products.iter().position(|p| p.id == product_id) {
                    self.products.remove(pos);
                }
			},
		}
	}	

  	pub fn view(&self) -> Element<'static, ProductsScreenMessage> {
		let rows = self.products.iter().fold(
			Column::new().push(
				Row::new()
				.spacing(20)
				.push(Container::new("ID").width(Length::FillPortion(1)).padding(8))
				.push(Container::new("Name").width(Length::FillPortion(4)).padding(8))
				.push(Container::new("Unity").width(Length::FillPortion(2)).padding(8))
				.push(Container::new("Min Stock").width(Length::FillPortion(2)).padding(8))
				.push(Container::new("Observation").width(Length::FillPortion(3)).padding(8))
				.push(Container::new("Actions").width(Length::FillPortion(1)).align_x(Alignment::Center).padding(8))
			),
			|column, product| {
				column.push(
					Row::new()
						.spacing(20)
						.push(Container::new(Text::new(product.id)).width(Length::FillPortion(1)).padding(8))
						.push(Container::new(Text::new(product.name.clone())).width(Length::FillPortion(4)).padding(8))
						.push(Container::new(Text::new(product.unity.clone().unwrap_or("".to_string()))).width(Length::FillPortion(2)).padding(8))
						.push(Container::new(Text::new(product.min_stock)).width(Length::FillPortion(2)).padding(8))
						.push(Container::new(Text::new(product.observation.clone().unwrap_or("".to_string()))).width(Length::FillPortion(3)).padding(8))
						.push(
							Container::new(
								Button::new(Text::new("Delete"))
									.style(|theme, status| iced::widget::button::danger(theme, status))
									.on_press(ProductsScreenMessage::Delete(product.id))
							).width(Length::FillPortion(1)).align_x(Alignment::Center).padding(8)
						)
				)
			}
		);

		Column::new()
			.width(Length::Fill)
			.padding(DEFAULT_SPACING)
			.spacing(DEFAULT_SPACING)
			.push(
				Row::new()
				.width(Length::Fill)
				.align_y(Alignment::Center)
				.push(Text::new("Products"))
				.push(Space::with_width(Length::Fill))
				.push(
					Button::new(Text::new("Add Product")).on_press(ProductsScreenMessage::Create)
				)
			)
			.push(
				Container::new(rows).center_x(Length::Fill)
			).into()
  	}
}

