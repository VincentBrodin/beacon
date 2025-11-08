use iced::widget::{Column, button, column, text};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("Iced failed: {0}")]
    Iced(#[from] iced::Error),
}

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl Counter {
    pub fn view(&'_ self) -> Column<'_, Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value).size(50),
            button("-").on_press(Message::Decrement),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}

fn main() -> Result<(), Error> {
    match iced::run("beacon", Counter::update, Counter::view) {
        Ok(()) => Ok(()),
        Err(err) => Err(Error::Iced(err)),
    }
}
