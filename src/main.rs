use config::{Config, PressedKeyCoord};

use error::LoadError;
use iced::event::{self, Event};
use iced::keyboard::{key, Modifiers};
use iced::widget::canvas::{Cache, Geometry, Path, Text};
use iced::widget::{canvas, column, container, text};
use iced::{mouse, Color, Point, Size};
use iced::{Element, Length, Rectangle, Renderer, Subscription, Task as Command, Theme};
use lesson::{Lesson, LessonAction, LessonPage};

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
    next_lesson_index: usize,
    current_lesson: Option<Lesson>,
    next_page_index: usize,
    current_page: Option<LessonPage>,
    next_action_index: usize,
    current_action: Option<LessonAction>,
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
    Event(Event),
    Tick,
}

impl RaitiApp {
    fn load() -> Command<Message> {
        Command::batch(vec![
            Command::perform(
                Config::load("./data/keyboards/querty.yaml"),
                Message::ConfigLoaded,
            ),
            Command::perform(Lesson::load("./data/lessons.yaml"), Message::LessonLoaded),
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
                    self.next_lesson_index = 0;
                    self.perform_lesson();
                    Command::none()
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
                                if key == iced::keyboard::Key::Named(key::Named::Enter) {
                                    self.perform_page();
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
            Message::Tick => {
                if let Some(lesson) = &self.current_page {
                    if let Some(action) = lesson.actions.get(self.next_action_index) {
                        self.current_action = Some(action.clone());
                        self.next_action_index += 1;
                    } else {
                        self.next_action_index = 0;
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        if self.config_loaded && self.current_lesson.is_some() && self.current_page.is_some() {
            let page = self.current_page.clone().unwrap();
            let title = text(page.title).size(25);
            let content = text(page.content);
            let cursor = if self.current_action.is_some()
                && self.current_action.clone().unwrap() == LessonAction::ShowCursor
            {
                text("_")
            } else {
                text(" ")
            };
            let content2 = text(page.content2);
            let keyboard = canvas(self as &Self)
                .width(Length::Fill)
                .height(Length::Fill);
            let page = column![title, content, cursor, content2, keyboard];

            container(page)
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
        Subscription::batch([
            event::listen().map(Message::Event),
            iced::time::every(std::time::Duration::from_millis(500)).map(|_| Message::Tick),
        ])
    }

    fn perform_lesson(&mut self) {
        if let Some(lesson) = self.lessons.get(self.next_lesson_index) {
            self.current_lesson = Some(lesson.clone());
            self.next_lesson_index += 1;
            self.next_page_index = 0;
            self.perform_page();
        } else {
            self.current_lesson = None;
        }
    }

    fn perform_page(&mut self) {
        if let Some(lesson) = self.lessons.get(self.next_lesson_index - 1) {
            if let Some(page) = lesson.pages.get(self.next_page_index) {
                self.current_page = Some(page.clone());
                self.next_page_index += 1;
                self.next_action_index = 0;
            } else {
                self.perform_lesson();
            }
        }
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
