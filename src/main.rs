use iced::event::{self, Event};
use iced::widget::canvas::path::Arc;
use iced::widget::canvas::{Cache, Geometry, Path};
use iced::widget::{canvas, column, container, svg, Svg};
use iced::{
    executor, Application, Command, Element, Length, Rectangle, Renderer, Settings, Subscription,
    Theme,
};
use iced::{mouse, Color, Point, Size};

fn main() -> iced::Result {
    RaitiApp::run(Settings::default())
}

struct RaitiApp {
    raiti_app: Cache,
}

#[derive(Debug, Clone)]
enum Message {
    Event(Event),
}

impl Application for RaitiApp {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                raiti_app: Cache::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Touch typing teaching app - Raiti")
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
        let keyboard = canvas(self as &Self)
            .width(Length::Fill)
            .height(Length::Fill);
        container(keyboard)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
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

            let keyboard = Path::new(|b| {
                b.move_to(Point {
                    x: CURVE,
                    y: keyboard_top_pad,
                });
                b.line_to(Point {
                    x: CURVE,
                    y: keyboard_top_pad,
                });
                b.line_to(Point {
                    x: keyboard_width - CURVE,
                    y: keyboard_top_pad,
                });
                b.arc_to(
                    Point {
                        x: keyboard_width,
                        y: keyboard_top_pad,
                    },
                    Point {
                        x: keyboard_width,
                        y: keyboard_top_pad + CURVE,
                    },
                    CURVE,
                );

                b.line_to(Point {
                    x: keyboard_width,
                    y: keyboard_top_pad + keyboard_height - CURVE,
                });
                b.arc_to(
                    Point {
                        x: keyboard_width,
                        y: keyboard_top_pad + keyboard_height,
                    },
                    Point {
                        x: keyboard_width - CURVE,
                        y: keyboard_top_pad + keyboard_height,
                    },
                    CURVE,
                );
                b.line_to(Point {
                    x: CURVE,
                    y: keyboard_top_pad + keyboard_height,
                });
                b.arc_to(
                    Point {
                        x: 0.0,
                        y: keyboard_top_pad + keyboard_height,
                    },
                    Point {
                        x: 0.0,
                        y: keyboard_top_pad + keyboard_height - CURVE,
                    },
                    CURVE,
                );
                b.line_to(Point { x: 0.0, y: keyboard_top_pad + CURVE });
                b.arc_to(Point { x: 0.0, y: keyboard_top_pad }, Point { x: CURVE, y: keyboard_top_pad }, CURVE);
                b.close();
            });
            // rectangle(
            //     Point {
            //         x: 0.0,
            //         y: keyboard_top_pad,
            //     },
            //     Size {
            //         width: keyboard_width,
            //         height: keyboard_height,
            //     },
            // );
            frame.fill(&keyboard, Color::from_rgb8(0x12, 0x93, 0xD8));
        });
        vec![keyboard]
    }
}
