use iced::widget::{container, text_editor};
use iced::{executor, Application, Command, Element, Settings, Subscription, Theme};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor {
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

impl Application for Editor {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                content: text_editor::Content::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Cool editor...")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Edit(action) => self.content.perform(action),
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let input = text_editor(&self.content).on_action(Message::Edit);
        container(input).padding(10).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        println!("Subscription event occured");
        Subscription::none()
    }
    
}
