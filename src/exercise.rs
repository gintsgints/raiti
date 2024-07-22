use iced::{widget::{text, column}, Element};

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Tick,
}

#[derive(Clone)]
pub struct Exercise {
    cursor_visible: bool,
    input: String,
}

impl Exercise {
    pub fn new() -> Exercise {
        Exercise {
            cursor_visible: false,
            input: "".to_string(),
        }
    }

    pub fn update(&mut self, message: Message) {
        if message == Message::Tick {
            self.cursor_visible = !self.cursor_visible;
        }
    }

    pub fn view<'a>(&'a self, exercise: &'a str) -> Element<'a, Message> {
        let ex = text(exercise);
        let done = if self.cursor_visible {
            text(format!("{}_", self.input))
        } else {
            text(self.input.clone())
        };
        column![ex, done].into()
    }
}
