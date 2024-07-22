use iced::{
    widget::{column, text},
    Element, Event,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Tick,
    Event(Event),
    Clear,
}

#[derive(Clone)]
pub struct Exercise {
    cursor_visible: bool,
    input: String,
}

impl Exercise {
    pub fn new() -> Exercise {
        Exercise {
            cursor_visible: false,
            input: "".to_string(),
        }
    }

    pub fn update(&mut self, message: Message) {
        #![allow(unused)]
        match message {
            Message::Tick => {
                self.cursor_visible = !self.cursor_visible;
            }
            Message::Event(event) => {
                if let Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key,
                    location,
                    modifiers,
                    text,
                }) = event
                {
                    // self.input.push(text.unwrap().into());
                    if let Some(ch) = text {
                        println!("Key pressed: {:?}. Location: {:?}", key, location);
                        match key {
                            iced::keyboard::Key::Character(_) => {
                                self.input.push_str(ch.as_str());
                            },
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::Backspace) => {
                                self.input.pop();   
                            },
                            _ => {}
                        }
                    }
                }
            }
            Message::Clear => {
                self.input.clear();
            },
        }
    }

    pub fn view<'a>(&'a self, exercise: &'a str) -> Element<'a, Message> {
        let ex = text(exercise);
        let done = if self.cursor_visible {
            text(format!("{}_", self.input))
        } else {
            text(format!("{} ", self.input))
        };
        column![ex, done].into()
    }
}
