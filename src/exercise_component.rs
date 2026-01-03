use std::{io::BufReader, thread, time::Duration};

use iced::{
    widget::{column, text},
    Element, Event,
};

use crate::{font, TICK_MILIS};

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Tick,
    Event(Event),
    SetFocus(bool),
}

pub struct ExerciseComponent {
    cursor_visible: bool,
    input: String,
    pub exercise: String,
    focus: bool,
    pub errors: u64,
    pub mseconds: u64,
}

impl ExerciseComponent {
    pub fn new(exercise: &str) -> ExerciseComponent {
        ExerciseComponent {
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
                    modified_key,
                    physical_key,
                }) = event
                {
                    if !self.focus {
                        return;
                    }
                    // println!("Key pressed: {:?}. Location: {:?}", key, location);
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

    pub fn view(&self) -> Element<'_, Message> {
        let ex = text(&self.exercise).size(20).font(font::MONO.clone());
        let done = if self.cursor_visible && (self.focus || self.exercise.is_empty()) {
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
            let stream_handle = rodio::OutputStreamBuilder::open_default_stream().unwrap();
            let mixer = stream_handle.mixer();

            let beep1 = {
                let file = std::fs::File::open("sounds/clack.mp3").unwrap();
                let sink = rodio::play(mixer, BufReader::new(file)).unwrap();
                sink.set_volume(0.2);
                sink
            };

            thread::sleep(Duration::from_millis(1500));

            drop(beep1);
        });
    }
}
