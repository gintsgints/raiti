use iced::{
    widget::{column, text},
    Element, Event,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Tick,
    Event(Event),
    Clear,
    SetFocus(bool),
}

#[derive(Clone, Default)]
pub struct Exercise {
    cursor_visible: bool,
    input: String,
    exercise: String,
    focus: bool,
}

impl Exercise {
    pub fn new(exercise: &str) -> Exercise {
        Exercise {
            exercise: exercise.to_string(),
            ..Default::default()
        }
    }

    pub fn update(&mut self, message: Message) {
        #![allow(unused)]
        match message {
            Message::Tick => {
                self.cursor_visible = !self.cursor_visible;
                if !self.exercise.starts_with(&self.input) {
                    self.input.pop();
                }        
            }
            Message::Event(event) => {
                if let Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key,
                    location,
                    modifiers,
                    text,
                }) = event
                {
                    if !self.focus {
                        return;
                    }
                    println!("Key pressed: {:?}. Location: {:?}", key, location);
                    if let Some(ch) = text {
                        match key {
                            iced::keyboard::Key::Character(_) => {
                                self.push_if_correct(ch.as_str());
                            },
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::Backspace) => {
                                self.input.pop();   
                            },
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) => {
                                self.push_if_correct(" ");
                            }
                            _ => {}
                        }
                    }
                }
            }
            Message::Clear => {
                self.input.clear();
                self.exercise.clear();
            }
            Message::SetFocus(focus) => {
                self.focus = focus;
            },
        }
    }

    pub fn view(&self) -> Element<Message> {
        let ex = text(&self.exercise).size(20);
        let done = if self.cursor_visible && self.focus {
            text(format!("{}_", self.input)).size(20)
        } else {
            text(format!("{} ", self.input)).size(20)
        };
        column![ex, done].padding(10).into()
    }

    pub fn exercise_finished(&self) -> bool {
        self.input.eq(&self.exercise)
    }

    fn push_if_correct(&mut self, letter: &str) {
        self.input.push_str(letter);
    }
}
