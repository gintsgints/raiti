use config::{Config, PressedKeyCoord};

use error::{LessonError, LoadError};
use iced::event::{self, Event};
use iced::keyboard::Modifiers;
use iced::widget::canvas::{Cache, Geometry, Path, Text};
use iced::widget::{canvas, column, container, text};
use iced::{mouse, Color, Point, Size};
use iced::{Element, Length, Rectangle, Renderer, Subscription, Task as Command, Theme};
use lesson::Lesson;

mod config;
mod error;
mod lesson;

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

#[derive(Default)]
struct RaitiApp {
    config_loaded: bool,
    lesson_loaded: bool,
    lessons: Vec<Lesson>,
    current_lesson: usize,
    error_loading: String,
    config: Config,
    raiti_app_draw_cache: Cache,
    modifiers: Modifiers,
    pressed_keys: Vec<PressedKeyCoord>,
}

#[derive(Debug)]
enum Message {
    ConfigLoaded(Result<Config, LoadError>),
    LessonLoaded(Result<Vec<Lesson>, LoadError>),
    NextLesson(Result<(), LessonError>),
    Event(Event),
}

impl RaitiApp {
    fn load() -> Command<Message> {
        Command::batch(vec![
            Command::perform(
                Config::load("./data/keyboards/querty.yaml"),
                Message::ConfigLoaded,
            ),
            Command::perform(
                Lesson::load("./data/lessons/base_keys.yaml"),
                Message::LessonLoaded,
            ),
        ])
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ConfigLoaded(result) => {
                match result {
                    Ok(config) => {
                        self.config_loaded = true;
                        self.config = config;
                    }
                    Err(error) => match error {
                        LoadError::File => self.error_loading = "Config file not found".to_string(),
                        LoadError::Format(err) => {
                            self.error_loading = format!("Error parsing config: {:}", err);
                        }
                    },
                };
                Command::none()
            }
            Message::LessonLoaded(result) => match result {
                Ok(lessons) => {
                    self.lesson_loaded = true;
                    self.lessons = lessons;
                    self.current_lesson = 0;

                    self.next_lesson()
                }
                Err(error) => {
                    match error {
                        LoadError::File => self.error_loading = "Lesson file not found".to_string(),
                        LoadError::Format(err) => {
                            self.error_loading = format!("Error parsing lesson file: {:}", err)
                        }
                    }
                    Command::none()
                }
            },
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
                                    self.config.find_key(key.clone(), location)
                                {
                                    self.pressed_keys.push(PressedKeyCoord { row, key });
                                    self.raiti_app_draw_cache.clear();
                                }
                            }
                            iced::keyboard::Event::KeyReleased {
                                key,
                                location,
                                modifiers,
                            } => {
                                if let Some((row, key)) = self.config.find_key(key, location) {
                                    self.pressed_keys
                                        .retain(|keys| !(keys.row == row && keys.key == key));
                                    self.raiti_app_draw_cache.clear();
                                }
                            }
                            iced::keyboard::Event::ModifiersChanged(modifiers) => {
                                self.modifiers = modifiers;
                                self.raiti_app_draw_cache.clear();
                            }
                        }
                    }
                    Event::Mouse(_) => {}
                    Event::Window(_) => {}
                    Event::Touch(_) => {}
                };
                Command::none()
            }
            Message::NextLesson(_) => {
                self.next_lesson()
            }
        }
    }

    fn next_lesson(&mut self) -> Command<Message> {
        let may_be_lesson = self.lessons.get(self.current_lesson);
        self.current_lesson += 1;
        match may_be_lesson {
            Some(lesson) => Command::perform(
                RaitiApp::perform_lesson(lesson.clone()),
                Message::NextLesson,
            ),
            None => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        if self.config_loaded && self.lesson_loaded {
            let keyboard = canvas(self as &Self)
                .width(Length::Fill)
                .height(Length::Fill);
            container(keyboard)
                .padding(30)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        } else {
            let loading_text = text("Loading ...");
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

    pub async fn perform_lesson(_lesson: Lesson) -> Result<(), LessonError> {
        Ok(())
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
        let key_fill_color = Color::from_rgb8(0xD1, 0xD1, 0xD1);
        let key_press_letter_color = Color::from_rgb8(0xFF, 0xFF, 0xFF);
        let key_press_fill_color = Color::from_rgb8(0x91, 0x91, 0x91);
        let second_label_y: f32 = 28.0;

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
                for (row_index, row) in self.config.rows.iter().enumerate() {
                    let mut key_x: f32 = self.config.keyboard_side_padding;
                    for (key_index, keyspec) in row.keys.iter().enumerate() {
                        let mut cur_letter_color = letter_color;
                        let mut cur_fill_color = key_fill_color;
                        for pressed_key in self.pressed_keys.iter() {
                            if pressed_key.row == row_index && pressed_key.key == key_index {
                                cur_letter_color = key_press_letter_color;
                                cur_fill_color = key_press_fill_color;
                            }
                        }

                        let key_pos = Point::new(key_x, key_y);
                        let key = Path::rounded_rectangle(
                            key_pos,
                            Size::new(simple_key_width * keyspec.width_ratio, simple_key_width),
                            self.config.key_corner_curve,
                        );
                        frame.fill(&key, cur_fill_color);
                        frame.fill_text(Text {
                            content: keyspec.label1.clone(),
                            position: Point::new(
                                key_x + self.config.key_text_left_pad,
                                key_y + self.config.key_text_top_pad,
                            ),
                            color: cur_letter_color,
                            ..canvas::Text::default()
                        });
                        if !keyspec.label2.is_empty() {
                            frame.fill_text(Text {
                                content: keyspec.label2.clone(),
                                position: Point::new(
                                    key_x + self.config.key_text_left_pad,
                                    key_y + self.config.key_text_top_pad + second_label_y,
                                ),
                                color: cur_letter_color,
                                ..canvas::Text::default()
                            });
                        }
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
