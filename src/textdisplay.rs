use bevy::{
    ecs::system::SystemParam,
    prelude::*,
};

pub struct TextDisplayPlugin;

#[rustfmt::skip]
impl Plugin for TextDisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NewMessage>();
    }
}

/// A struct that represents a message that should be displayed.
#[derive(Clone, Debug)]
pub enum Message {
    /// Free-form text.
    Text(String),

    /// The name of an item that has just been picked up.
    ItemPickup(String),

    /// The result of a successful item combination (`source -> destination = result`).
    ItemCombine {
        /// Source item name
        src: String,

        /// Destination item name
        dst: String, 

        /// Result item name
        result: String 
    },

    /// The result of an unsuccesful item combination.
    InvalidItemCombination,

    /// A message sent when the wrong item is used on an Interactive.
    InvalidItemUsed,
}

impl Message {
    /// Creates a `Message::Text` with the passed string.
    pub fn new(text: &str) -> Self {
        Self::Text(text.to_owned())
    }
}

/// An event that triggers whenever an `Action::Message` is executed.
#[derive(Debug)]
pub struct NewMessage(pub Message);

#[derive(SystemParam)]
pub struct TextDisplay<'w, 's> {
    events: EventWriter<'w, 's, NewMessage>,
}

impl<'w, 's> TextDisplay<'w, 's> {
    pub fn show(&mut self, message: Message) {
        self.events.send(NewMessage(message));
    }
}
