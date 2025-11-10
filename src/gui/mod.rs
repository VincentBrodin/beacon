use std::{cmp, sync::LazyLock, time::Instant};

use iced::{
    Application, Command, Element, Font, Theme, executor,
    font::Weight,
    keyboard::{self, Key, Modifiers, key::Named},
    widget::{
        Column, Container, Space, Text, TextInput, column,
        text::LineHeight,
        text_input::{self, Id},
    },
    window,
};
use nucleo::{
    Nucleo,
    pattern::{CaseMatching, Normalization},
};

use crate::{
    desktop::Desktop,
    gui::style::{ItemContainer, ItemContainerStyle},
};

mod style;

static SEARCH_INPUT_ID: LazyLock<Id> = LazyLock::new(|| Id::new("search_input"));

#[derive(Debug, Clone)]
pub enum Message {
    SetQuery(String),
    KeyPressed((Key, Modifiers)),
    Submit,
}

pub struct App {
    query: String,
    selected: usize,
    count: usize,
    nucleo: Nucleo<Desktop>,
}

pub struct Flags {
    pub nucleo: Nucleo<Desktop>,
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = Flags;
    type Message = Message;
    type Theme = Theme;

    fn new(flags: Flags) -> (App, Command<Self::Message>) {
        return (
            Self {
                query: String::default(),
                selected: 0,
                count: 5,
                nucleo: flags.nucleo,
            },
            text_input::focus(SEARCH_INPUT_ID.clone()),
        );
    }

    fn title(&self) -> String {
        String::from("beacon")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SetQuery(val) => {
                println!("User entered: {}", val);
                self.query = val;
                let now = Instant::now();
                self.nucleo.pattern.reparse(
                    0,
                    &self.query,
                    CaseMatching::Ignore,
                    Normalization::Smart,
                    false,
                );
                let status = self.nucleo.tick(15);
                let duration = now.elapsed();
                println!(
                    "Matched and sorted for {} in {:?} with status {:?}",
                    self.query, duration, status
                );
                self.count = self.nucleo.snapshot().matched_item_count() as usize;
                self.selected = 0;
                Command::none()
            }
            Message::KeyPressed((key, modifiers)) => match key {
                Key::Character(key) => {
                    let char = match modifiers.shift() {
                        true => key.as_str().to_uppercase(),
                        false => key.as_str().to_string(),
                    };
                    println!("User pressed {}", char);
                    self.query.push_str(&char);
                    text_input::focus(SEARCH_INPUT_ID.clone())
                }
                Key::Named(key) => {
                    if key == Named::Escape {
                        window::close(window::Id::MAIN)
                    } else if key == Named::Tab {
                        println!("TAB");
                        // let snapshot = self.nucleo.snapshot();
                        if self.selected + 1 >= 5.min(self.count) {
                            self.selected = 0
                        } else {
                            self.selected += 1;
                        }
                        Command::none()
                    } else if key == Named::Enter {
                        println!("ENTER");
                        Command::none()
                    } else {
                        Command::none()
                    }
                }
                Key::Unidentified => Command::none(),
            },
            Message::Submit => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let snapshot = self.nucleo.snapshot();
        let mut results = Column::<Message>::new();
        for (i, item) in snapshot
            .matched_items(0..cmp::min(5, snapshot.matched_item_count()))
            .enumerate()
        {
            let mut text = Text::new(item.data.name.to_string())
                .size(16)
                .line_height(LineHeight::Relative(1.5));
            // let mut container = Container::new(text);
            if i == self.selected {
                let mut font = Font::DEFAULT;
                font.weight = Weight::Bold;
                text = text.font(font);
                // container = Container::new(text);
            }
            let container = Container::new(text);
            results = results.push(container);
        }

        let input = TextInput::new("Search", &self.query)
            .on_input(|new_val| Message::SetQuery(new_val))
            .on_submit(Message::Submit)
            .id(SEARCH_INPUT_ID.clone());

        let _ = text_input::focus::<Self::Message>(SEARCH_INPUT_ID.clone());
        column![input, Space::with_height(8), results]
            .padding(16)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        keyboard::on_key_press(|key_code, modifiers| {
            Some(Message::KeyPressed((key_code, modifiers)))
        })
    }

    fn theme(&self) -> Self::Theme {
        Theme::KanagawaWave
    }
}
