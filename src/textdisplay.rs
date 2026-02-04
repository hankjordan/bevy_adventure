use std::marker::PhantomData;

use bevy::prelude::*;
// use bevy::derive::SystemParam; // Not needed if in prelude

use crate::Item;

pub struct TextDisplayPlugin;

impl Plugin for TextDisplayPlugin {
    fn build(&self, app: &mut App) {
        app ////
            .add_message::<NewMessage>();
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

impl bevy::ecs::message::Message for NewMessage {} 
// If Message trait exists and NewMessage needs it:
// impl bevy::ecs::message::Message for NewMessage {} 
// But "Message" trait might be "Event"? 
// Error 766 said "NewMessage is not a Message". 
// Maybe MessageWriter expects T: Message?
// I will try to leave it as Event and see if imports fixes it?
// Actually, earlier error 766 happened WITH derive(Event).
// So I MUST derive Message or impl it.
// I'll try derive(Event) + impl Message manually.

#[derive(bevy::ecs::system::SystemParam)]
pub struct TextDisplay<'w, 's> {
    events: MessageWriter<'w, NewMessage>,

    _marker: PhantomData<&'s ()>,
}

impl<'w, 's> TextDisplay<'w, 's> {
    pub fn show(&mut self, message: Message) {
        self.events.write(NewMessage(message));
    }
}
