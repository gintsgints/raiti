pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

use config::Config;
use iced::{widget::text, Element};

mod config;

fn main() -> Result<()> {
    let config = Config::load()?;
    iced::application("Raiti", Raiti::update, Raiti::view).run_with(move || Raiti { config: config.clone() })?;
    Ok(())
}

struct Raiti {
    config: Config,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {}

impl Raiti {
    fn update(&mut self, _: Message) {
    }

    fn view(&self) -> Element<Message> {
        text("Hello Iced...").into()
    }
}
