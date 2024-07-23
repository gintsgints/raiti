pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

use config::{Config, IndexRecord};
use exercise::Exercise;
use iced::{
    event,
    keyboard::key,
    widget::{button, column, container, text},
    window, Element, Event, Length, Subscription, Task,
};
use keyboard::Keyboard;

mod config;
mod environment;
mod exercise;
mod keyboard;

fn main() -> Result<()> {
    // Read config & initialize state
    let config = Config::load()?;

    iced::application("Raiti", Raiti::update, Raiti::view)
        .subscription(Raiti::subscription)
        .run_with(move || Raiti {
            config: config.clone(),
            exercise: vec![],
            keyboard: Keyboard::new(config.keyboard.clone()),
            show_confirm: false,
        })?;
    Ok(())
}

struct Raiti {
    config: Config,
    exercise: Vec<Exercise>,
    keyboard: Keyboard,
    show_confirm: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(Event),
    Tick,
    Exercise(exercise::Message),
    Keyboard(keyboard::Message),
    LessonSelected(IndexRecord),
    Confirm,
    WindowSettingsSaved(core::result::Result<(), config::Error>),
}

impl Raiti {
    fn update(&mut self, message: Message) -> Task<Message> {
        #![allow(unused)]
        match message {
            Message::Exercise(message) => {
                for exercise in self.exercise.iter_mut() {
                    exercise.update(message.clone());
                }
                Task::none()
            }
            Message::Event(event) => {
                for exercise in self.exercise.iter_mut() {
                    exercise.update(exercise::Message::Event(event.clone()));
                }
                self.keyboard
                    .update(keyboard::Message::Event(event.clone()));
                if let Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key,
                    location,
                    modifiers,
                    text,
                }) = event
                {
                    match key {
                        iced::keyboard::Key::Named(key::Named::Enter) => {
                            if self.show_confirm {
                                return self.exit_with_save();
                            }
                            let finished = self.exercise.iter().all(|ex| ex.exercise_finished());
                            if finished {
                                self.exercise.clear();
                                self.keyboard.update(keyboard::Message::ClearKeys);
                                self.config.next_page();
                                if let Some(page) = self.config.get_page() {
                                    if !page.show_keys.is_empty() {
                                        self.keyboard.update(keyboard::Message::SetShowKeys(
                                            page.show_keys.clone(),
                                        ))
                                    }
                                }
                                if let Some(ex) = self.config.get_exercise() {
                                    match ex {
                                        config::Exercise::None => {},
                                        config::Exercise::OneLineNoEnter(line) => {
                                            self.exercise.push(Exercise::new(line));
                                        },
                                        config::Exercise::Multiline(lines) => {
                                            for line in lines.lines() {
                                                let mut ex = Exercise::new(line);
                                                if self.exercise.is_empty() {
                                                    ex.update(exercise::Message::SetFocus(true))
                                                }
                                                self.exercise.push(ex);
                                            }
                                        },
                                    }
                                }
                            } else {
                                for exercise in self.exercise.iter_mut() {
                                    if !exercise.exercise_finished() {
                                        exercise.update(exercise::Message::SetFocus(true));
                                        break;
                                    } else {
                                        exercise.update(exercise::Message::SetFocus(false));
                                    }
                                }
                            }
                        }
                        iced::keyboard::Key::Named(key::Named::Escape) => {
                            self.show_confirm = !self.show_confirm;
                        }
                        _ => {}
                    }
                }
                Task::none()
            }
            Message::Tick => {
                for exercise in self.exercise.iter_mut() {
                    exercise.update(exercise::Message::Tick);
                }
                
                self.keyboard.update(keyboard::Message::Tick);
                Task::none()
            }
            Message::Keyboard(message) => {
                self.keyboard.update(message);
                Task::none()
            }
            Message::LessonSelected(lesson) => {
                // TODO: find a way to fail lesson load without unwrap
                self.config.load_lesson(&lesson.file).unwrap();
                Task::none()
            }
            Message::Confirm => self.exit_with_save(),
            Message::WindowSettingsSaved(result) => {
                if let Err(err) = result {
                    println!("window settings failed to save: {:?}", err);
                }
                window::get_latest().and_then(window::close)
            }
        }
    }

    fn view(&self) -> Element<Message> {
        if self.show_confirm {
            let content = column![
                "Are you sure you want to exit?",
                button("Yes, exit now")
                    .padding([10, 20])
                    .on_press(Message::Confirm),
            ]
            .spacing(10);
            return container(content)
                .padding(30)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into();
        }
        if let Some(page) = self.config.get_page() {
            let title = text(&page.title).size(25);
            let mut page_content = column![title];
            page_content = page_content.push(text(&page.content));
            page_content = if page.keyboard {
                page_content.push(self.keyboard.view().map(Message::Keyboard))
            } else {
                page_content
            };

            for exercise in self.exercise.iter() {
                page_content = page_content.push(exercise.view().map(Message::Exercise));
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
            for lesson in &self.config.index.lessons {
                let btn =
                    button(text(&lesson.title)).on_press(Message::LessonSelected(lesson.clone()));
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
            iced::time::every(std::time::Duration::from_millis(500)).map(|_| Message::Tick),
        ])
    }

    fn exit_with_save(&self) -> Task<Message> {
        Task::perform(self.config.clone().save(), Message::WindowSettingsSaved)
    }
}
