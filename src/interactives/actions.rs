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

#[derive(Clone, Debug)]
pub enum Action<State> {
    AddItem(AddedItem),
    Animation(String),
    Message(Message),
    Transition(State),
    Move(String),
}

impl<State> Action<State> {
    pub fn single(self) -> Vec<Self> {
        vec![self]
    }
}

pub fn invalid_item<State>() -> Vec<Action<State>> {
    vec![Action::Message(Message::InvalidItemUsed)]
}

pub fn invalid_combine() -> Message {
    Message::InvalidItemCombination
}

#[allow(unused_variables)]
pub trait Interactive {
    type State: StateData;

    fn use_item(
        &mut self,
        state: &mut ResMut<WorldState>,
        item: &mut ItemRef,
    ) -> Vec<Action<Self::State>> {
        invalid_item()
    }

    fn update(&mut self, commands: &mut CommandsExt, state: &mut ResMut<WorldState>) {}

    fn interact(&mut self, state: &mut ResMut<WorldState>) -> Vec<Action<Self::State>>;
}
