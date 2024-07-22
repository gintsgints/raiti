pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

use config::{Config, PressedKeyCoord};
use exercise::Exercise;
use iced::{
    event,
    keyboard::key,
    widget::{column, container, text},
    Element, Event, Length, Subscription,
};

mod config;
mod environment;
mod exercise;

fn main() -> Result<()> {
    // Read config & initialize state
    let config = Config::load()?;

    iced::application("Raiti", Raiti::update, Raiti::view)
        .subscription(Raiti::subscription)
        .run_with(move || Raiti {
            config: config.clone(),
            pressed_keys: vec![],
            exercise: Exercise::new(),
        })?;
    Ok(())
}

struct Raiti {
    config: Config,
    pressed_keys: Vec<PressedKeyCoord>,
    exercise: Exercise,
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(Event),
    Tick,
    Exercise(exercise::Message),
}

impl Raiti {
    fn update(&mut self, message: Message) {
        match message {
            Message::Exercise(message) => {
                self.exercise.update(message)
            }
            Message::Event(event) => {
                match event {
                    Event::Keyboard(event) => {
                        match event {
                            #![allow(unused)]
                            iced::keyboard::Event::KeyPressed {
                                key,
                                location,
                                modifiers,
                                text,
                            } => {
                                println!("Key pressed: {:?}. Location: {:?}", key, location);
                                if let Some((row, key)) =
                                    self.config.keyboard.find_key(key.clone(), location)
                                {
                                    self.pressed_keys.push(PressedKeyCoord { row, key });
                                    // self.raiti_app_draw_cache.clear();
                                }
                                if key == iced::keyboard::Key::Named(key::Named::Enter) {
                                    self.config.next_page();
                                }
                            }
                            iced::keyboard::Event::KeyReleased {
                                key,
                                location,
                                modifiers,
                            } => {
                                if let Some((row, key)) =
                                    self.config.keyboard.find_key(key, location)
                                {
                                    self.pressed_keys
                                        .retain(|keys| !(keys.row == row && keys.key == key));
                                    // self.raiti_app_draw_cache.clear();
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Message::Tick => {
                self.exercise.update(exercise::Message::Tick)
            }
        }
    }

    fn view(&self) -> Element<Message> {
        if let Some(page) = self.config.get_page() {
            let title = text(&page.title).size(25);
            let content = text(&page.content);
            let content2 = text(&page.content2);

            let page_content = if let Some(ex) = self.config.get_exercise() {
                match ex {
                    config::Exercise::None => {column![title, content, content2]},
                    config::Exercise::OneLineNoEnter(line) => {
                        column![title, content, self.exercise.view(line).map(Message::Exercise), content2]
                    },
                }
            } else {
                column![title, content, content2]
            };
            //     self.exercise.set_text(self.config.get_exercise().unwrap());
            //     // let exercise = exercise(self.config.get_exercise().unwrap(), Message::Event);
            //     // column![title, content, exercise, content2]
            //     column![title, content, content2]
            // } else {
            //     column![title, content, content2]
            // };
            container(page_content)
                .padding(30)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        } else {
            text("Lesson finished").into()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            event::listen().map(Message::Event),
            iced::time::every(std::time::Duration::from_millis(500)).map(|_| Message::Tick),
        ])
    }
}
