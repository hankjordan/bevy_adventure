use std::marker::PhantomData;

use bevy::{
    ecs::system::SystemParam,
    prelude::*,
};

use crate::Item;

pub struct TextDisplayPlugin;

impl Plugin for TextDisplayPlugin {
    fn build(&self, app: &mut App) {
        app ////
            .add_event::<NewMessage>();
    }
}

/// A struct that represents a message that should be displayed.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum Message {
    /// Free-form text.
    Text(String),

    /// The Item that has just been picked up.
    ItemPickup(Item),

    /// The result of a successful item combination (`source -> destination = result`).
    ItemCombine {
        /// Source Item
        src: Item,

        /// Destination Item
        dst: Item,

        /// Result Item
        result: Item,
    },

    /// The result of an unsuccesful Item combination.
    InvalidItemCombination,

    /// A message sent when the wrong Item is used on an Interactive.
    InvalidItemUsed,
}

impl Message {
    /// Creates a `Message::Text` with the passed string.
    pub fn new(text: &str) -> Self {
        Self::Text(text.to_owned())
    }
}

/// An event that triggers whenever an `Action::Message` is executed.
#[derive(Debug, Event)]
pub struct NewMessage(pub Message);

#[derive(SystemParam)]
pub struct TextDisplay<'w, 's> {
    events: EventWriter<'w, NewMessage>,

    #[system_param(ignore)]
    _marker: PhantomData<&'s ()>,
}

impl<'w, 's> TextDisplay<'w, 's> {
    pub fn show(&mut self, message: Message) {
        self.events.send(NewMessage(message));
    }
}
