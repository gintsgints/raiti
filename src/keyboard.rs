use iced::widget::canvas;
use iced::{
    border::Radius,
    mouse,
    widget::canvas::{Cache, Geometry, Path, Text},
    Color, Element, Event, Length, Point, Rectangle, Renderer, Size, Theme,
};

use crate::config::PressedKeyCoord;

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Event(Event),
    Tick,
    ClearKeys,
    SetShowKeys(Vec<PressedKeyCoord>),
}

#[derive(Default)]
pub struct Keyboard {
    draw_cache: Cache,
    config: crate::config::Keyboard,
    pressed_keys: Vec<PressedKeyCoord>,
    show_keys: Vec<PressedKeyCoord>,
    key_to_show: usize,
    hide: bool,
}

impl Keyboard {
    pub fn new(config: crate::config::Keyboard) -> Keyboard {
        Keyboard {
            config,
            ..Default::default()
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Event(event) => {
                if let Event::Keyboard(event) = event {
                    match event {
                        #![allow(unused)]
                        iced::keyboard::Event::KeyPressed {
                            key,
                            location,
                            modifiers,
                            text,
                        } => {
                            if let Some((row, key)) = self.config.find_key(key.clone(), location) {
                                self.pressed_keys.push(PressedKeyCoord { row, key });
                                self.draw_cache.clear();
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
                                self.draw_cache.clear();
                            }
                        }
                        _ => {}
                    }
                }
            }
            Message::SetShowKeys(keys) => {
                self.show_keys = keys;
            }
            Message::Tick => {
                if !self.show_keys.is_empty() {
                    if let Some(key) = self.show_keys.get(self.key_to_show) {
                        if self.hide {
                            self.pressed_keys
                                .retain(|keys| !(keys.row == key.row && keys.key == key.key));
                            self.key_to_show += 1;
                        } else {
                            self.pressed_keys.push(key.clone());
                        }
                        self.hide = !self.hide;
                    } else {
                        self.key_to_show = 0;
                        self.hide = false;
                    }
                    self.draw_cache.clear();
                }
            }
            Message::ClearKeys => {
                self.show_keys.clear();
                self.pressed_keys.clear();
                self.key_to_show = 0;
                self.hide = false;
            },
        }
    }

    pub fn view(&self) -> Element<Message> {
        canvas(self as &Self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl<Message> canvas::Program<Message> for Keyboard {
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

        let keyboard = self.draw_cache.draw(renderer, bounds.size(), |frame| {
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
                Radius::from(self.config.keyboard_corner_curve),
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
                        Radius::from(self.config.keyboard_corner_curve),
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
