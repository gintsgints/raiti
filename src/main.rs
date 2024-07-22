pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

use config::{Config, PressedKeyCoord};
use exercise::Exercise;
use iced::{
    event,
    keyboard::key,
    widget::{canvas, column, container, text},
    Element, Event, Length, Subscription,
};
use keyboard::Keyboard;

mod config;
mod environment;
mod exercise;
mod keyboard;

fn main() -> Result<()> {
    // Read config & initialize state
    let config = Config::load()?;

    iced::application("Raiti", Raiti::update, Raiti::view)
        .subscription(Raiti::subscription)
        .run_with(move || Raiti {
            config: config.clone(),
            pressed_keys: vec![],
            exercise: Exercise::new(),
            keyboard: Keyboard::new(config.keyboard.clone()),
        })?;
    Ok(())
}

struct Raiti {
    config: Config,
    pressed_keys: Vec<PressedKeyCoord>,
    exercise: Exercise,
    keyboard: Keyboard,
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(Event),
    Tick,
    Exercise(exercise::Message),
    Keyboard(keyboard::Message),
}

impl Raiti {
    fn update(&mut self, message: Message) {
        #![allow(unused)]
        match message {
            Message::Exercise(message) => self.exercise.update(message),
            Message::Event(event) => {
                self.exercise
                    .update(exercise::Message::Event(event.clone()));
                self.keyboard
                    .update(keyboard::Message::Event(event.clone()));
                if let Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key,
                    location,
                    modifiers,
                    text,
                }) = event
                {
                    if key == iced::keyboard::Key::Named(key::Named::Enter) {
                        self.exercise.update(exercise::Message::Clear);
                        self.config.next_page();
                    }
                }
            }
            Message::Tick => self.exercise.update(exercise::Message::Tick),
            Message::Keyboard(message) => self.keyboard.update(message),
        }
    }

    fn view(&self) -> Element<Message> {
        if let Some(page) = self.config.get_page() {
            let title = text(&page.title).size(25);
            let mut page_content = column![title];
            page_content = page_content.push(text(&page.content));
            page_content = if page.keyboard {
                page_content.push(self.keyboard.view().map(Message::Keyboard))
            } else {
                page_content
            };

            page_content = if let Some(ex) = self.config.get_exercise() {
                match ex {
                    config::Exercise::None => page_content,
                    config::Exercise::OneLineNoEnter(line) => {
                        page_content.push(self.exercise.view(line).map(Message::Exercise))
                    }
                }
            } else {
                page_content
            };
            page_content = page_content.push(text(&page.content2));

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
