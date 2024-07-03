use iced::event::{self, Event};
use iced::widget::canvas::{Cache, Geometry, Path};
use iced::widget::{canvas, container};
use iced::{
    Element, Length, Rectangle, Renderer, Subscription, Theme,
};
use iced::{mouse, Color, Point, Size};

fn main() -> iced::Result {
    iced::application(
        "Touch Typing learn system",
        RaitiApp::update,
        RaitiApp::view,
    )
    .subscription(RaitiApp::subscription)
    .theme(|_| Theme::Dark)
    .antialiasing(true)
    .run()
}

#[derive(Default)]
struct RaitiApp {
    raiti_app: Cache,
}

#[derive(Debug, Clone)]
enum Message {
    Event(Event),
}

impl RaitiApp {
    fn update(&mut self, message: Message) {
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
                Event::Window(_) => {}
                Event::Touch(_) => {}
            },
        };
    }

    fn view(&self) -> Element<'_, Message> {
        let keyboard = canvas(self as &Self)
            .width(Length::Fill)
            .height(Length::Fill);
        container(keyboard)
            .padding(20)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }
}

impl<Message> canvas::Program<Message> for RaitiApp {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        pub const ROWS_FOR_KEYS: f32 = 23.0;
        pub const CURVE: f32 = 10.0;

        let keyboard = self.raiti_app.draw(renderer, bounds.size(), |frame| {
            let keyboard_width = frame.width();
            let simple_key_width = keyboard_width / ROWS_FOR_KEYS;
            let keyboard_height = simple_key_width * 7.0;
            let keyboard_top_pad = (frame.height() - keyboard_height) / 2.0;

            let keyboard = Path::rounded_rectangle(
                Point {
                    x: 0.0,
                    y: keyboard_top_pad,
                },
                Size {
                    width: keyboard_width,
                    height: keyboard_height,
                },
                CURVE,
            );
            frame.fill(&keyboard, Color::from_rgb8(0x12, 0x93, 0xD8));
        });
        vec![keyboard]
    }
}
