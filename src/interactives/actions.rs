use bevy::{
    ecs::schedule::StateData,
    prelude::*,
};

use crate::{
    commands::CommandsExt,
    interactives::ItemRef,
    inventory::AddedItem,
    state::WorldState,
    textdisplay::Message,
};

/// An enum of possible actions an Interactive might take after being interacted with in some way.
#[derive(Clone, Debug)]
pub enum Action<State> {
    /// Add an item to the player's inventory.
    AddItem(AddedItem),

    /// Play an animation.
    Animation(String),

    /// Send a message.
    Message(Message),

    /// Change the current state to the given state.
    Transition(State),

    /// Move to a CameraSpot with the given name.
    Move(String),
}

impl<State> Action<State> {
    /// Turns an Action<State> into a Vec<Action<State>> with a single item.
    pub fn single(self) -> Vec<Self> {
        vec![self]
    }
}

/// Trait that allows you to define behavior for an object in a Scene.
#[allow(unused_variables)]
pub trait Interactive {
    /// The type of the state that the interactive is a part of.
    /// Must match the Scene's state type for the Interactive to be registered properly.
    type State: StateData;

    /// Optional method called whenever an item is used on an object.
    ///
    /// Returns a Vec of Actions defining what happens after the item is used.
    fn use_item(
        &mut self,
        state: &mut ResMut<WorldState>,
        item: &mut ItemRef,
    ) -> Vec<Action<Self::State>> {
        Action::Message(Message::InvalidItemUsed).single()
    }

    /// Optional method used to modify entities in the world, called every frame.
    fn update(&mut self, commands: &mut CommandsExt, state: &mut ResMut<WorldState>) {}

    /// Method called whenever an object is interacted with.
    ///
    /// Returns a Vec of Actions defining what happens as a result of the interaction.
    fn interact(&mut self, state: &mut ResMut<WorldState>) -> Vec<Action<Self::State>>;
}
