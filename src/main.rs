use iced::{Element};
use iced::widget::{container, text};

#[derive(Default)]
struct StockManagement;

#[derive(Debug, Clone, Copy)]
enum Message {}

impl StockManagement {
    fn update(&mut self, message: Message) {
    }

    fn view(&self) -> Element<Message> {
        let hello = text("Stock Management!");
        container(hello).into()
    }
}


fn main() -> iced::Result {
    iced::run(
        "Stock Management",
        StockManagement::update,
        StockManagement::view
    )
}
