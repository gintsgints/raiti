use config::{Config, LoadError};

use iced::event::{self, Event};
use iced::keyboard::Modifiers;
use iced::widget::canvas::{Cache, Geometry, Path, Text};
use iced::widget::{canvas, column, container, text};
use iced::{mouse, Color, Point, Size};
use iced::{Element, Length, Rectangle, Renderer, Subscription, Task as Command, Theme};

mod config;

fn main() -> iced::Result {
    iced::application(
        "Touch Typing learn system",
        RaitiApp::update,
        RaitiApp::view,
    )
    .load(RaitiApp::load)
    .subscription(RaitiApp::subscription)
    .theme(|_| Theme::Dark)
    .antialiasing(true)
    .run()
}

struct PressedKeyCoord {
    x: usize,
    y: usize,
}

#[derive(Default)]
struct RaitiApp {
    loaded: bool,
    error_loading: String,
    config: Config,
    raiti_app_draw_cache: Cache,
    modifiers: Modifiers,
    pressed_keys: Vec<PressedKeyCoord>,
}

#[derive(Debug)]
enum Message {
    Loaded(Result<Config, LoadError>),
    Event(Event),
}

impl RaitiApp {
    fn load() -> Command<Message> {
        Command::perform(
            Config::load("./data/keyboards/querty.yaml"),
            Message::Loaded,
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Loaded(result) => match result {
                Ok(config) => {
                    self.loaded = true;
                    self.config = config;
                }
                Err(error) => match error {
                    LoadError::File => self.error_loading = "Config file not found".to_string(),
                    LoadError::Format(err) => {
                        self.error_loading = format!("Error parsing config: {:}", err)
                    }
                },
            },
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
                        } => {
                            println!("Key pressed: {:?}", key)
                        }
                        iced::keyboard::Event::KeyReleased {
                            key,
                            location,
                            modifiers,
                        } => {}
                        iced::keyboard::Event::ModifiersChanged(modifiers) => {
                            self.modifiers = modifiers;
                            self.raiti_app_draw_cache.clear();
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
        if self.loaded {
            let keyboard = canvas(self as &Self)
                .width(Length::Fill)
                .height(Length::Fill);
            container(keyboard)
                .padding(30)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        } else {
            let loading_text = text("Loading keyboard ...");
            let error_text = text(self.error_loading.clone());
            let result_text = column![loading_text, error_text];
            container(result_text)
                .padding(30)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        }
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
        let letter_color = Color::BLACK;
        let key_press_letter_color = Color::from_rgb8(0xFF, 0xFF, 0xFF);

        let keyboard = self
            .raiti_app_draw_cache
            .draw(renderer, bounds.size(), |frame| {
                let keyboard_width = frame.width();
                let simple_key_width = keyboard_width / self.config.cols_for_keys;
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
                    self.config.keyboard_corner_curve,
                );
                frame.fill(&keyboard, Color::from_rgb8(0xFF, 0xFF, 0xFF));

                let mut key_y: f32 = keyboard_top_pad + self.config.keyboard_side_padding;
                for (key_y_index, row) in self.config.rows.iter().enumerate() {
                    let mut key_x: f32 = self.config.keyboard_side_padding;
                    for (key_x_index, keyspec) in row.keys.iter().enumerate() {
                        let mut cur_letter_color = letter_color;
                        for pressed_key in self.pressed_keys.iter() {
                            if pressed_key.x == key_x_index && pressed_key.y == key_y_index {
                                cur_letter_color = key_press_letter_color;
                            }
                        }

                        let key_pos = Point::new(key_x, key_y);
                        let key = Path::rounded_rectangle(
                            key_pos,
                            Size::new(simple_key_width * keyspec.width_ratio, simple_key_width),
                            self.config.key_corner_curve,
                        );
                        frame.fill(&key, Color::from_rgb8(0xD1, 0xD1, 0xD1));
                        frame.fill_text(Text {
                            content: keyspec.label1.clone(),
                            position: Point::new(
                                key_x + self.config.key_text_left_pad,
                                key_y + self.config.key_text_top_pad,
                            ),
                            color: cur_letter_color,
                            ..canvas::Text::default()
                        });
                        key_x = key_x
                            + self.config.keyboard_side_padding
                            + simple_key_width * keyspec.width_ratio;
                    }
                    key_y = key_y + simple_key_width + self.config.space_between_keys;
                }
            });
        vec![keyboard]
    }
}
