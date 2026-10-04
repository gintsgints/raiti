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
        self, button, canvas::path::lyon_path::geom::euclid::num::Round, column, container, image,
        row, scrollable, text,
    },
    window, Alignment, Element, Event, Length, Subscription, Task,
};
use serde_json::json;

use crate::config::Lesson;

mod beeper;
mod config;
mod data;
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
    /// A lesson could not be loaded. Holds the message to show.
    Error(String),
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
    /// The table of contents covers the lesson while it is shown.
    show_contents: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(Event),
    Tick,
    Exercise(exercise_component::Message),
    Keyboard(keyboard_component::Message),
    LessonSelected(IndexRecord),
    PageSelected(usize),
    Confirm(DialogType),
    DismissDialog,
    WindowSettingsSaved(core::result::Result<(), config::Error>),
}

impl Raiti {
    fn new() -> (Self, Task<Message>) {
        // Read config & initialize state
        let config = Config::load().expect("Error loading context");
        let keyboard_config = config
            .read_keyboard()
            .unwrap_or_else(|e| panic!("Error loading keyboard config: {e}"));

        // A lesson that cannot be loaded is reported rather than fatal, so the
        // rest of the course stays usable.
        let (lesson, dialog) = match config.current_lesson() {
            Some(Ok(lesson)) => (Some(lesson), DialogType::None),
            Some(Err(e)) => (None, DialogType::Error(e.to_string())),
            None => (None, DialogType::None),
        };

        let mut raiti = Self {
            config: config.clone(),
            lesson,
            exercise_components: vec![],
            keyboard: KeyboardComponent::new(keyboard_config),
            dialog,
            ..Default::default()
        };

        raiti.construct_exercise_components();

        (raiti, widget::focus_next())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        #![allow(unused)]
        match message {
            Message::Exercise(message) => {
                for exercise_component in &mut self.exercise_components {
                    exercise_component.update(message.clone());
                }
                Task::none()
            }
            Message::Event(event) => {
                // While the contents are up they take every key, so nothing
                // typed there reaches the exercises underneath.
                if self.show_contents {
                    if let Event::Keyboard(iced::keyboard::Event::KeyPressed {
                        key,
                        modifiers,
                        ..
                    }) = event
                    {
                        if matches!(key, iced::keyboard::Key::Named(key::Named::Escape)) {
                            // Leaving the contents drops the lesson rather
                            // than entering it.
                            self.show_contents = false;
                            self.exercise_components.clear();
                            self.lesson = None;
                        } else if Self::is_contents_shortcut(&key, modifiers) {
                            self.show_contents = false;
                        }
                    }
                    return Task::none();
                }
                for exercise_component in &mut self.exercise_components {
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
                        iced::keyboard::Key::Named(key::Named::ArrowDown)
                            if modifiers.contains(Modifiers::SHIFT)
                                && modifiers.contains(Modifiers::ALT) =>
                        {
                            self.move_next_page();
                        }
                        iced::keyboard::Key::Named(key::Named::ArrowUp)
                            if modifiers.contains(Modifiers::SHIFT)
                                && modifiers.contains(Modifiers::ALT) =>
                        {
                            self.open_contents();
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
                                .all(exercise_component::ExerciseComponent::exercise_finished);
                            if finished {
                                self.move_next_page();
                            }
                            for exercise_component in &mut self.exercise_components {
                                if exercise_component.exercise_finished() {
                                    exercise_component
                                        .update(exercise_component::Message::SetFocus(false));
                                } else {
                                    exercise_component
                                        .update(exercise_component::Message::SetFocus(true));
                                    break;
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
                for exercise_component in &mut self.exercise_components {
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
                self.lesson = self.load_lesson(&lesson.file);
                // A lesson opens on its contents, so a reader can start in the
                // middle of it.
                self.open_contents();
                Task::none()
            }
            Message::PageSelected(page_index) => {
                self.move_to_page(page_index);
                Task::none()
            }
            Message::Confirm(dialog_type) => match dialog_type {
                // The error dialog is dismissed, never confirmed.
                DialogType::None | DialogType::Error(_) => Task::none(),
                DialogType::ConfirmExitLesson => {
                    self.lesson = None;
                    self.dialog = DialogType::None;
                    Task::none()
                }
                DialogType::ConfirmExitApp => self.exit_with_save(),
            },
            Message::DismissDialog => {
                self.dialog = DialogType::None;
                Task::none()
            }
            Message::WindowSettingsSaved(result) => {
                if let Err(err) = result {
                    println!("window settings failed to save: {err:?}");
                }
                window::get_latest().and_then(window::close)
            }
        }
    }

    /// Renders the dialog covering the page, if one is up.
    fn dialog_view(&self) -> Option<Element<'_, Message>> {
        let content = match &self.dialog {
            DialogType::None => return None,
            DialogType::ConfirmExitLesson => column![
                text("Are you sure you want to exit lesson?"),
                button("Yes, exit lesson")
                    .padding([10, 20])
                    .on_press(Message::Confirm(DialogType::ConfirmExitLesson)),
            ],
            DialogType::ConfirmExitApp => column![
                text("Are you sure you want to exit app?"),
                button("Yes, exit app")
                    .padding([10, 20])
                    .on_press(Message::Confirm(DialogType::ConfirmExitApp)),
            ],
            DialogType::Error(message) => column![
                text("Lesson could not be loaded").size(25),
                text(message),
                button("Close")
                    .padding([10, 20])
                    .on_press(Message::DismissDialog),
            ],
        };

        Some(
            container(content.spacing(10))
                .padding(30)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
        )
    }

    /// Renders the lesson table of contents, if it is up.
    fn contents_view(&self) -> Option<Element<'_, Message>> {
        if !self.show_contents {
            return None;
        }
        let lesson = self.lesson.as_ref()?;
        let entries = lesson.menu_entries();
        // The page being read is rarely an entry itself, so the section it
        // belongs to is the last entry that starts at or before it.
        let current_entry = entries
            .iter()
            .rev()
            .find(|(page_index, _)| *page_index <= self.config.current_page)
            .map(|(page_index, _)| *page_index);

        let mut list = column![].spacing(8);
        for (page_index, title) in entries {
            let label = if Some(page_index) == current_entry {
                text(format!("> {title}"))
            } else {
                text(format!("  {title}"))
            };
            list = list.push(button(label).on_press(Message::PageSelected(page_index)));
        }

        let contents = column![
            text(lesson.title().unwrap_or("Lesson")).size(25),
            scrollable(list),
            text("Choose lesson. <Esc> for the lesson list.").size(12),
        ]
        .spacing(15);

        Some(
            container(contents)
                .padding(30)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
        )
    }

    fn view(&self) -> Element<'_, Message> {
        if let Some(dialog) = self.dialog_view() {
            return dialog;
        }

        if let Some(contents) = self.contents_view() {
            return contents;
        }

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
            let content: Element<'_, Message> = match &page.image_handle {
                Some(handle) => {
                    let mut illustration = image(handle.clone());
                    if let Some(width) = page.image_width {
                        illustration = illustration.width(width);
                    }
                    row![
                        text(rendered_content.clone()),
                        container(illustration).padding(20)
                    ]
                    .spacing(30)
                    .align_y(Alignment::Center)
                    .into()
                }
                None => text(rendered_content.clone()).into(),
            };
            page_content = page_content.push(content);
            page_content = if page.keyboard {
                page_content.push(self.keyboard.view().map(Message::Keyboard))
            } else {
                page_content
            };

            for exercise_component in &self.exercise_components {
                page_content = page_content.push(exercise_component.view().map(Message::Exercise));
            }
            page_content = page_content.push(text(&page.content2));
            if self.has_contents() {
                page_content =
                    page_content.push(text("<Shift>+<Alt>+<Up> for lesson contents").size(12));
            }

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
                                ex.update(exercise_component::Message::SetFocus(true));
                            }
                            self.exercise_components.push(ex);
                        }
                    }
                }
            }
        }
    }

    /// Loads a lesson, raising the error dialog instead of returning a lesson
    /// when it cannot be read.
    fn load_lesson(&mut self, file_name: &str) -> Option<Lesson> {
        match self.config.load_lesson(file_name) {
            Ok(lesson) => Some(lesson),
            Err(e) => {
                self.dialog = DialogType::Error(e.to_string());
                None
            }
        }
    }

    /// Whether the current lesson offers a table of contents.
    fn has_contents(&self) -> bool {
        self.lesson
            .as_ref()
            .is_some_and(|lesson| !lesson.menu_entries().is_empty())
    }

    fn is_contents_shortcut(key: &iced::keyboard::Key, modifiers: Modifiers) -> bool {
        matches!(key, iced::keyboard::Key::Named(key::Named::ArrowUp))
            && modifiers.contains(Modifiers::SHIFT)
            && modifiers.contains(Modifiers::ALT)
    }

    fn open_contents(&mut self) {
        self.show_contents = self.dialog == DialogType::None && self.has_contents();
    }

    /// Jumps to a page picked from the table of contents.
    fn move_to_page(&mut self, page_index: usize) {
        self.show_contents = false;
        self.config.current_page = page_index;
        self.config.current_exercise = 0;
        self.show_page();
    }

    /// Rebuilds keyboard hints and exercises for the current page. Returns
    /// false when the lesson holds no such page.
    fn show_page(&mut self) -> bool {
        self.exercise_components.clear();
        self.keyboard.update(keyboard_component::Message::ClearKeys);
        let Some(show_keys) = self
            .lesson
            .as_ref()
            .and_then(|lesson| lesson.get_page(self.config.current_page))
            .map(|page| page.show_keys.clone())
        else {
            return false;
        };
        if !show_keys.is_empty() {
            self.keyboard
                .update(keyboard_component::Message::SetShowKeys(show_keys));
        }
        self.construct_exercise_components();
        true
    }

    fn move_next_page(&mut self) {
        self.calculate_stats();

        self.config.next_page();
        if !self.show_page() && self.lesson.is_some() {
            // The lesson ran out of pages, so it is finished.
            self.lesson = self
                .config
                .index
                .next_lesson(&self.config.current_lesson)
                .map(String::from)
                .and_then(|name| self.load_lesson(&name));
            self.config.current_exercise = 0;
            self.config.current_page = 0;
            self.open_contents();
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
