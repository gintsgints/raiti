use iced::event::{self, Event};
use iced::widget::canvas::{Cache, Geometry, Path, Text};
use iced::widget::{canvas, container};
use iced::{mouse, Color, Point, Size};
use iced::{Element, Length, Rectangle, Renderer, Subscription, Theme};

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
            .padding(30)
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
        pub const SMALL_KEY_SPACE: f32 = 5.0;
        pub const KEYBOARD_CURVE: f32 = 8.0;
        pub const KEYBOARD_PAD: f32 = 5.0;
        pub const KEY_CURVE: f32 = 3.0;
        pub const KEY_TEXT_TOP_PAD: f32 = 5.0;
        pub const KEY_TEXT_LEFT_PAD: f32 = 3.0;

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
                KEYBOARD_CURVE,
            );
            frame.fill(&keyboard, Color::from_rgb8(0xFF, 0xFF, 0xFF));

            let ctrl_key_pos = Point::new(
                SMALL_KEY_SPACE + KEYBOARD_PAD,
                keyboard_top_pad + keyboard_height - SMALL_KEY_SPACE - simple_key_width - KEYBOARD_PAD,
            );

            let ctrl_key = Path::rounded_rectangle(ctrl_key_pos, Size::new(simple_key_width, simple_key_width), KEY_CURVE);
            frame.fill(&ctrl_key, Color::from_rgb8(0xD1, 0xD1, 0xD1));
            frame.fill_text(Text {
                content: "Ctrl".to_string(),
                position: Point::new(ctrl_key_pos.x + KEY_TEXT_LEFT_PAD, ctrl_key_pos.y + KEY_TEXT_TOP_PAD),
                ..canvas::Text::default()
            });

            let alt_key_pos = Point::new(
                SMALL_KEY_SPACE + ctrl_key_pos.x + simple_key_width,
                keyboard_top_pad + keyboard_height - SMALL_KEY_SPACE - simple_key_width - KEYBOARD_PAD,
            );

            let alt_key = Path::rounded_rectangle(alt_key_pos, Size::new(simple_key_width, simple_key_width), KEY_CURVE);
            frame.fill(&alt_key, Color::from_rgb8(0xD1, 0xD1, 0xD1));

        });
        vec![keyboard]
    }
}
