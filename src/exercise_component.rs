use std::time::Instant;

use iced::{
    widget::{column, text},
    Element, Event,
};

use crate::{beeper::Beeper, font};

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Tick,
    Event(Event),
    SetFocus(bool),
}

pub struct ExerciseComponent {
    cursor_visible: bool,
    input: String,
    pub exercise: String,
    focus: bool,
    pub errors: u64,
    /// Time spent typing this exercise: from the first character entered to
    /// the last one. Time spent reading the page before typing starts does not
    /// count, and neither does the pause after the exercise is complete.
    pub mseconds: u64,
    first_key: Option<Instant>,
    beeper: Beeper,
}

impl ExerciseComponent {
    pub fn new(exercise: &str) -> ExerciseComponent {
        ExerciseComponent {
            exercise: exercise.to_string(),
            cursor_visible: false,
            input: String::new(),
            focus: false,
            errors: 0,
            mseconds: 0,
            first_key: None,
            beeper: Beeper::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        #![allow(unused)]
        match message {
            Message::Tick => {
                self.cursor_visible = !self.cursor_visible;
            }
            Message::Event(event) => {
                if let Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key,
                    location,
                    modifiers,
                    text,
                    modified_key,
                    physical_key,
                }) = event
                {
                    if !self.focus {
                        return;
                    }
                    // println!("Key pressed: {:?}. Location: {:?}", key, location);
                    if let Some(ch) = text {
                        match key {
                            iced::keyboard::Key::Character(_) => self.type_in(ch.as_str()),
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::Backspace) => {
                                self.input.pop();
                            }
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) => {
                                self.type_in(" ");
                            }
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::Tab) => {
                                self.type_in("  ");
                            }
                            _ => {}
                        }
                    }
                }
            }
            Message::SetFocus(focus) => {
                self.focus = focus;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let ex = text(&self.exercise).size(20).font(font::MONO.clone());
        let done = if self.cursor_visible && (self.focus || self.exercise.is_empty()) {
            text(format!("{}_", self.input))
                .size(20)
                .font(font::MONO.clone())
        } else {
            text(format!("{} ", self.input))
                .size(20)
                .font(font::MONO.clone())
        };
        column![ex, done].padding(10).into()
    }

    pub fn exercise_finished(&self) -> bool {
        self.input.eq(&self.exercise)
    }

    /// Takes in what a key stands for. A wrong one never reaches the typed
    /// line: it beeps, counts as a single error and leaves the cursor where it
    /// was, so the line on screen always holds the exercise so far.
    fn type_in(&mut self, typed: &str) {
        let mut candidate = self.input.clone();
        candidate.push_str(typed);
        if self.exercise.starts_with(&candidate) {
            self.input = candidate;
            self.record_typing_time();
        } else {
            self.errors += 1;
            self.beeper.play_beep();
        }
    }

    /// The first character starts the clock, every later one moves the end of
    /// it, so the measured time ends with the character that finishes the
    /// exercise.
    fn record_typing_time(&mut self) {
        let first_key = *self.first_key.get_or_insert_with(Instant::now);
        self.mseconds = u64::try_from(first_key.elapsed().as_millis()).unwrap_or(u64::MAX);
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use iced::keyboard::{key::Named, Key, Location, Modifiers};

    use super::*;

    fn key_press(character: char) -> Message {
        let text = character.to_string();
        let key = if character == ' ' {
            Key::Named(Named::Space)
        } else {
            Key::Character(text.clone().into())
        };
        Message::Event(Event::Keyboard(iced::keyboard::Event::KeyPressed {
            key: key.clone(),
            modified_key: key,
            physical_key: iced::keyboard::key::Physical::Unidentified(
                iced::keyboard::key::NativeCode::Unidentified,
            ),
            location: Location::Standard,
            modifiers: Modifiers::empty(),
            text: Some(text.into()),
        }))
    }

    fn type_in(component: &mut ExerciseComponent, input: &str) {
        for character in input.chars() {
            component.update(key_press(character));
        }
    }

    #[test]
    fn a_wrong_key_is_one_error_and_never_lands_in_the_typed_line() {
        let mut component = ExerciseComponent::new("ab");
        component.update(Message::SetFocus(true));
        type_in(&mut component, "axb");
        assert_eq!(component.input, "ab");
        assert_eq!(component.errors, 1);
        assert!(component.exercise_finished());
    }

    #[test]
    fn every_wrong_key_counts_on_its_own() {
        let mut component = ExerciseComponent::new("ab");
        component.update(Message::SetFocus(true));
        type_in(&mut component, "xyz");
        assert_eq!(component.input, "");
        assert_eq!(component.errors, 3);
    }

    #[test]
    fn a_wrong_key_does_not_start_the_clock() {
        let mut component = ExerciseComponent::new("ab");
        component.update(Message::SetFocus(true));
        type_in(&mut component, "x");
        assert_eq!(component.mseconds, 0);
    }

    #[test]
    fn an_untouched_exercise_takes_no_time() {
        let mut component = ExerciseComponent::new("abc");
        component.update(Message::SetFocus(true));
        for _ in 0..10 {
            component.update(Message::Tick);
        }
        assert_eq!(component.mseconds, 0);
    }

    #[test]
    fn only_the_time_between_the_first_and_the_last_character_counts() {
        let mut component = ExerciseComponent::new("ab");
        component.update(Message::SetFocus(true));
        sleep(Duration::from_millis(30));
        type_in(&mut component, "a");
        sleep(Duration::from_millis(30));
        type_in(&mut component, "b");
        sleep(Duration::from_millis(30));
        assert!(component.exercise_finished());
        assert!(
            (30..90).contains(&component.mseconds),
            "expected about 30 ms of typing, got {}",
            component.mseconds
        );
    }

    #[test]
    fn an_exercise_nobody_types_in_stays_at_zero_while_another_one_is_typed() {
        let mut typed = ExerciseComponent::new("ab");
        let mut idle = ExerciseComponent::new("cd");
        typed.update(Message::SetFocus(true));
        type_in(&mut typed, "a");
        sleep(Duration::from_millis(20));
        type_in(&mut typed, "b");
        // Both components see every event, but only the focused one counts.
        type_in(&mut idle, "a");
        idle.update(Message::Tick);
        assert_eq!(idle.mseconds, 0);
        assert!(typed.mseconds > 0);
    }
}
