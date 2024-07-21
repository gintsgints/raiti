use iced::{
    widget::{component, text, Component},
    Element, Event,
};

pub struct Exercise<Message> {
    text: String,
    input: String,
    on_change: Box<dyn Fn(Event) -> Message>,
}

pub fn exercise<Message>(
    exercise: &crate::config::Exercise,
    on_change: impl Fn(Event) -> Message + 'static,
) -> Exercise<Message> {
    match exercise {
        crate::config::Exercise::None => Exercise::new("", on_change),
        crate::config::Exercise::OneLineNoEnter(ex) => Exercise::new(ex, on_change),
    }
}

impl<Message> Exercise<Message> {
    pub fn new(text: &str, on_change: impl Fn(Event) -> Message + 'static) -> Self {
        Self {
            text: text.to_string(),
            input: "".to_string(),
            on_change: Box::new(on_change),
        }
    }
}

impl<Message, Theme> Component<Message, Theme> for Exercise<Message>
where
    Theme: text::Catalog + 'static,
{
    type State = ();

    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        println!("{:?}", event);
        None
    }

    fn view(&self, state: &Self::State) -> Element<'_, Self::Event, Theme> {
        text(self.text.clone()).into()
    }
}

impl<'a, Message, Theme> From<Exercise<Message>> for Element<'a, Message, Theme>
where
    Theme: text::Catalog + 'static,
    Message: 'a,
{
    fn from(exercise: Exercise<Message>) -> Self {
        component(exercise)
    }
}
