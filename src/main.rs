use iced::event::{self, Event};
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
    Event(Event),
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
            Message::Event(event) => {
                match event {
                    Event::Keyboard(event) => {
                        match event {
                            #![allow(unused)]
                            iced::keyboard::Event::KeyPressed { key, location, modifiers, text } => {},
                            iced::keyboard::Event::KeyReleased { key, location, modifiers } => {},
                            iced::keyboard::Event::ModifiersChanged(modifiers) => {
                                println!("Modifiers changed: {:?}", modifiers);
                            },
                        }
                    },
                    Event::Mouse(_) => {},
                    Event::Window(_, _) => {},
                    Event::Touch(_) => {},
                }
            }
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let input = text_editor(&self.content).on_action(Message::Edit);
        container(input).padding(10).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }
}
