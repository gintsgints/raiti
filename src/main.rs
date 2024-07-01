use iced::event::{self, Event};
use iced::widget::{container, svg};
use iced::{executor, Application, Command, Element, Settings, Subscription, Theme};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor {}

#[derive(Debug, Clone)]
enum Message {
    Event(Event),
}

impl Application for Editor {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Self {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("Cool editor...")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Event(event) => match event {
                Event::Keyboard(event) =>
                {
                    match event {
                        #![allow(unused)]
                        iced::keyboard::Event::KeyPressed {
                            key,
                            location,
                            modifiers,
                            text,
                        } => {}
                        iced::keyboard::Event::KeyReleased {
                            key,
                            location,
                            modifiers,
                        } => {}
                        iced::keyboard::Event::ModifiersChanged(modifiers) => {
                            println!("Modifiers changed: {:?}", modifiers);
                        }
                    }
                }
                Event::Mouse(_) => {}
                Event::Window(_, _) => {}
                Event::Touch(_) => {}
            },
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let handle = svg::Handle::from_path("./img/simple_key.svg");
        let svg = svg(handle).height(60).width(60);

        container(svg).center_x().center_y().into()
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }
}
