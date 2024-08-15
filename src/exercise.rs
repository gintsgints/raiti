use std::{io::BufReader, thread};

use iced::{
    widget::{column, text},
    Element, Event,
};
use rodio::{OutputStream, Sink};

use crate::{font, TICK_MILIS};

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Tick,
    Event(Event),
    SetFocus(bool),
}

pub struct Exercise {
    cursor_visible: bool,
    input: String,
    pub exercise: String,
    focus: bool,
    pub errors: u64,
    pub mseconds: u64,
}

impl Exercise {
    pub fn new(exercise: &str) -> Exercise {
        Exercise {
            exercise: exercise.to_string(),
            cursor_visible: false,
            input: "".to_string(),
            focus: false,
            errors: 0,
            mseconds: 0,
        }
    }

    pub fn update(&mut self, message: Message) {
        #![allow(unused)]
        match message {
            Message::Tick => {
                self.mseconds += TICK_MILIS;
                self.cursor_visible = !self.cursor_visible;
                if !self.exercise.starts_with(&self.input) {
                    self.beep();
                }
                while !self.exercise.starts_with(&self.input) && !self.input.is_empty() {
                    self.errors += 1;
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
                                self.input.push_str(ch.as_str());
                            }
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::Backspace) => {
                                self.input.pop();
                            }
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) => {
                                self.input.push(' ');
                            }
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::Tab) => {
                                self.input.push_str("  ");
                            }
                            _ => {}
                        }
                    }
                }
            }
            Message::SetFocus(focus) => {
                self.focus = focus;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let ex = text(&self.exercise).size(20).font(font::MONO.clone());
        let done = if self.cursor_visible && self.focus {
            text(format!("{}_", self.input))
                .size(20)
                .font(font::MONO.clone())
        } else {
            text(format!("{} ", self.input))
                .size(20)
                .font(font::MONO.clone())
        };
        column![ex, done].padding(10).into()
    }

    pub fn exercise_finished(&self) -> bool {
        self.input.eq(&self.exercise)
    }

    fn beep(&self) {
        thread::spawn(move || {
            let (_stream, handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&handle).unwrap();

            let file = std::fs::File::open("sounds/clack.mp3").unwrap();
            sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
            sink.sleep_until_end();
        });
    }
}
