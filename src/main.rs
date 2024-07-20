pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

use config::Config;
use iced::{
    widget::{column, container, text},
    Element, Length,
};

mod config;
mod environment;

fn main() -> Result<()> {
    // Read config & initialize state
    let config = Config::load()?;

    iced::application("Raiti", Raiti::update, Raiti::view).run_with(move || Raiti {
        config: config.clone(),
    })?;
    Ok(())
}

struct Raiti {
    config: Config,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {}

impl Raiti {
    fn update(&mut self, _: Message) {
        println!("{:?}", self.config.keyboard);
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
}
