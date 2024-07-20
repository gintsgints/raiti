pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

use config::{Config, PressedKeyCoord};
use iced::{
    event, keyboard::key, widget::{column, container, text}, Element, Event, Length, Subscription
};

mod config;
mod environment;

fn main() -> Result<()> {
    // Read config & initialize state
    let config = Config::load()?;

    iced::application("Raiti", Raiti::update, Raiti::view)
    .subscription(Raiti::subscription)
    .run_with(move || Raiti {
        config: config.clone(),
        pressed_keys: vec![],
    })?;
    Ok(())
}

struct Raiti {
    config: Config,
    pressed_keys: Vec<PressedKeyCoord>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(Event),
    Tick,
}

impl Raiti {
    fn update(&mut self, message: Message) {
        match message {
            Message::Event(event) => {
                match event {
                    Event::Keyboard(event) =>
                    {
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
                                    self.config.perform_page();
                                }
                            }
                            iced::keyboard::Event::KeyReleased {
                                key,
                                location,
                                modifiers,
                            } => {
                                if let Some((row, key)) = self.config.keyboard.find_key(key, location) {
                                    self.pressed_keys
                                        .retain(|keys| !(keys.row == row && keys.key == key));
                                    // self.raiti_app_draw_cache.clear();
                                }
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            },
            Message::Tick => {
                
            },
        }
    }

    fn view(&self) -> Element<Message> {
        if let Some(page) = self.config.lesson.pages.get(self.config.next_page) {
            let title = text(&page.title).size(25);
            let content = text(&page.content);
            let content2 = text(&page.content2);
            let page_content = column![title, content, content2];
            container(page_content)
                .padding(30)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        } else {
            text(format!("{:?}", self.config.lesson)).into()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            event::listen().map(Message::Event),
            iced::time::every(std::time::Duration::from_millis(500)).map(|_| Message::Tick),
        ])
    }
}
