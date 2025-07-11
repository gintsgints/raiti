pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

use config::{Config, IndexRecord};
use exercise_component::ExerciseComponent;
use keyboard_component::KeyboardComponent;

use handlebars::Handlebars;
use iced::{
    event,
    keyboard::{key, Modifiers},
    widget::{
        self, button, canvas::path::lyon_path::geom::euclid::num::Round, column, container, text,
    },
    window, Element, Event, Length, Subscription, Task,
};
use serde_json::json;

use crate::{config::Lesson, keyboard_config::KeyboardConfig};

mod config;
mod environment;
mod exercise_component;
mod font;
mod keyboard_component;
mod keyboard_config;

pub const TICK_MILIS: u64 = 500;

fn main() -> iced::Result {
    font::set();

    iced::application("Raiti - Touch typing tutor", Raiti::update, Raiti::view)
        .subscription(Raiti::subscription)
        .settings(iced::Settings {
            id: None,
            antialiasing: false,
            fonts: font::load(),
            ..Default::default()
        })
        .run_with(Raiti::new)
}

#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub enum DialogType {
    #[default]
    None,
    ConfirmExitLesson,
    ConfirmExitApp,
}

#[derive(Default)]
struct Raiti {
    config: Config,
    lesson: Option<Lesson>,
    exercise_components: Vec<ExerciseComponent>,
    was_errors: u64,
    was_wpm: f64,
    keyboard: KeyboardComponent,
    dialog: DialogType,
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(Event),
    Tick,
    Exercise(exercise_component::Message),
    Keyboard(keyboard_component::Message),
    LessonSelected(IndexRecord),
    Confirm(DialogType),
    WindowSettingsSaved(core::result::Result<(), config::Error>),
}

impl Raiti {
    fn new() -> (Self, Task<Message>) {
        // Read config & initialize state
        let config = Config::load().expect("Error loading context");
        let keyboard_config = KeyboardConfig::load(
            Config::data_dir()
                .join("keyboards")
                .join(format!("{}.yaml", &config.current_keyboard)),
        )
        .expect("Error loading keyboard config");

        let lesson = if !config.current_lesson.is_empty() {
            Some(
                Lesson::load(Config::data_dir().join(format!("{}.yaml", config.current_lesson)))
                    .expect("Error loading lesson"),
            )
        } else {
            None
        };

        let mut raiti = Self {
            config: config.clone(),
            lesson,
            exercise_components: vec![],
            keyboard: KeyboardComponent::new(keyboard_config),
            ..Default::default()
        };

        raiti.construct_exercise_components();

        (raiti, widget::focus_next())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        #![allow(unused)]
        match message {
            Message::Exercise(message) => {
                for exercise_component in self.exercise_components.iter_mut() {
                    exercise_component.update(message.clone());
                }
                Task::none()
            }
            Message::Event(event) => {
                for exercise_component in self.exercise_components.iter_mut() {
                    exercise_component.update(exercise_component::Message::Event(event.clone()));
                }
                self.keyboard
                    .update(keyboard_component::Message::Event(event.clone()));
                if let Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key,
                    location,
                    modifiers,
                    text,
                    modified_key,
                    physical_key,
                }) = event
                {
                    match key {
                        iced::keyboard::Key::Named(key::Named::ArrowDown) => {
                            if modifiers.contains(Modifiers::SHIFT)
                                && modifiers.contains(Modifiers::ALT)
                            {
                                self.move_next_page();
                            }
                        }
                        iced::keyboard::Key::Named(key::Named::Enter) => {
                            if self.dialog == DialogType::ConfirmExitApp {
                                return self.exit_with_save();
                            }
                            if self.dialog == DialogType::ConfirmExitLesson {
                                self.dialog = DialogType::None;
                                self.lesson = None;
                                return Task::none();
                            }
                            let finished = self
                                .exercise_components
                                .iter()
                                .all(|ex| ex.exercise_finished());
                            if finished {
                                self.move_next_page();
                            }
                            for exercise_component in self.exercise_components.iter_mut() {
                                if !exercise_component.exercise_finished() {
                                    exercise_component
                                        .update(exercise_component::Message::SetFocus(true));
                                    break;
                                } else {
                                    exercise_component
                                        .update(exercise_component::Message::SetFocus(false));
                                }
                            }
                        }
                        iced::keyboard::Key::Named(key::Named::Escape) => {
                            if self.dialog == DialogType::None {
                                if self.lesson.is_some() {
                                    self.dialog = DialogType::ConfirmExitLesson;
                                } else {
                                    self.dialog = DialogType::ConfirmExitApp;
                                }
                            } else {
                                self.dialog = DialogType::None;
                            }
                        }
                        _ => {}
                    }
                }
                Task::none()
            }
            Message::Tick => {
                for exercise_component in self.exercise_components.iter_mut() {
                    exercise_component.update(exercise_component::Message::Tick);
                }

                self.keyboard.update(keyboard_component::Message::Tick);
                Task::none()
            }
            Message::Keyboard(message) => {
                self.keyboard.update(message);
                Task::none()
            }
            Message::LessonSelected(lesson) => {
                self.exercise_components.clear();
                // TODO: find a way to fail lesson load without unwrap
                self.lesson = Some(
                    self.config
                        .load_lesson(&lesson.file)
                        .expect("Error loading lesson on selection"),
                );
                Task::none()
            }
            Message::Confirm(dialog_type) => match dialog_type {
                DialogType::None => Task::none(),
                DialogType::ConfirmExitLesson => {
                    self.lesson = None;
                    self.dialog = DialogType::None;
                    Task::none()
                }
                DialogType::ConfirmExitApp => self.exit_with_save(),
            },
            Message::WindowSettingsSaved(result) => {
                if let Err(err) = result {
                    println!("window settings failed to save: {:?}", err);
                }
                window::get_latest().and_then(window::close)
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self.dialog {
            DialogType::None => {}
            DialogType::ConfirmExitLesson => {
                let content = column![
                    "Are you sure you want to exit lesson?",
                    button("Yes, exit lesson")
                        .padding([10, 20])
                        .on_press(Message::Confirm(DialogType::ConfirmExitLesson)),
                ]
                .spacing(10);
                return container(content)
                    .padding(30)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .into();
            }
            DialogType::ConfirmExitApp => {
                let content = column![
                    "Are you sure you want to exit app?",
                    button("Yes, exit app")
                        .padding([10, 20])
                        .on_press(Message::Confirm(DialogType::ConfirmExitApp)),
                ]
                .spacing(10);
                return container(content)
                    .padding(30)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .into();
            }
        };
        if let Some(lesson) = &self.lesson {
            let page = lesson
                .get_page(self.config.current_page)
                .expect("No page found at view");
            let title = text(&page.title).size(25);
            let mut page_content = column![title];
            let reg = Handlebars::new();
            let rendered_content = reg
                .render_template(
                    &page.content,
                    &json!({"wpm": self.was_wpm, "errors": self.was_errors}),
                )
                .unwrap();
            page_content = page_content.push(text(rendered_content.clone()));
            page_content = if page.keyboard {
                page_content.push(self.keyboard.view().map(Message::Keyboard))
            } else {
                page_content
            };

            for exercise_component in self.exercise_components.iter() {
                page_content = page_content.push(exercise_component.view().map(Message::Exercise));
            }
            page_content = page_content.push(text(&page.content2));

            container(page_content)
                .padding(30)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        } else {
            let title = text("Please choose next lesson");
            let mut list = column![title].spacing(15);
            for index_record in &self.config.index.lessons {
                let btn = button(text(&index_record.title))
                    .on_press(Message::LessonSelected(index_record.clone()));
                list = list.push(btn);
            }
            container(list)
                .padding(30)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            event::listen().map(Message::Event),
            iced::time::every(std::time::Duration::from_millis(TICK_MILIS)).map(|_| Message::Tick),
        ])
    }

    fn exit_with_save(&self) -> Task<Message> {
        Task::perform(self.config.clone().save(), Message::WindowSettingsSaved)
    }

    fn construct_exercise_components(&mut self) {
        if let Some(lesson) = &self.lesson {
            if let Some(ex) =
                lesson.get_exercise(self.config.current_page, self.config.current_exercise)
            {
                match ex {
                    config::Exercise::None => {}
                    config::Exercise::OneLineNoEnter(line) => {
                        self.exercise_components.push(ExerciseComponent::new(line));
                    }
                    config::Exercise::Multiline(lines) => {
                        for line in lines.lines() {
                            let mut ex = ExerciseComponent::new(line);
                            if self.exercise_components.is_empty() {
                                ex.update(exercise_component::Message::SetFocus(true))
                            }
                            self.exercise_components.push(ex);
                        }
                    }
                }
            };
        }
    }

    fn move_next_page(&mut self) {
        self.calculate_stats();

        self.exercise_components.clear();
        self.keyboard.update(keyboard_component::Message::ClearKeys);
        self.config.next_page();
        if let Some(lesson) = &self.lesson {
            if let Some(page) = lesson.get_page(self.config.current_page) {
                if !page.show_keys.is_empty() {
                    self.keyboard
                        .update(keyboard_component::Message::SetShowKeys(
                            page.show_keys.clone(),
                        ))
                }
                self.construct_exercise_components();
            } else {
                self.lesson = self
                    .config
                    .index
                    .next_lesson(&self.config.current_lesson)
                    .map(String::from)
                    .map(|name| {
                        self.config
                            .load_lesson(&name)
                            .expect("Enable to load next lesson")
                    });
                self.config.current_exercise = 0;
                self.config.current_page = 0;
            }
        }
    }

    fn calculate_stats(&mut self) {
        let mut errors: u64 = 0;
        let mut mseconds: u64 = 0;
        let mut length: u64 = 0;
        for ex in &self.exercise_components {
            errors += ex.errors;
            mseconds += ex.mseconds;
            length += ex.exercise.chars().map(|_| 1).sum::<u64>();
        }
        self.was_errors = errors.round();
        let was_wpm = ((length as f64 - errors as f64) / (mseconds as f64 / 60000.0)) / 5.0;
        self.was_wpm = (was_wpm * 100.0).round() / 100.0;
    }
}
