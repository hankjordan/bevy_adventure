use bevy::{
    ecs::schedule::StateData,
    prelude::*,
};

use crate::{
    interactives::{
        Action,
        Interactive,
    },
    state::WorldState,
    textdisplay::Message,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoState;

/// A preset Interactive that displays a message when interacted with.
#[derive(Component)]
pub struct Description {
    text: String,
}

impl Description {
    /// Returns a new instance of `Description` with the given text.
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_owned(),
        }
    }
}

impl Interactive for Description {
    type State = NoState;

    fn interact(&mut self, _state: &mut ResMut<WorldState>) -> Vec<Action<Self::State>> {
        Action::Message(Message::new(&self.text)).single()
    }
}

/// A preset Interactive that moves to a `CameraSpot` when interacted with.
#[derive(Component)]
pub struct MoveTo {
    spot: String,
}

impl MoveTo {
    /// Returns a new instance of `MoveTo` with the given spot name.
    pub fn new(spot: &str) -> Self {
        Self {
            spot: spot.to_owned(),
        }
    }
}

impl Interactive for MoveTo {
    type State = NoState;

    fn interact(&mut self, _state: &mut ResMut<WorldState>) -> Vec<Action<Self::State>> {
        Action::Move(self.spot.clone()).single()
    }
}

/// A preset Interactive that changes the current state when interacted with.
#[derive(Component)]
pub struct Portal<T> {
    state: T,
}

impl<T> Portal<T> {
    /// Returns a new instance of `Portal` with the given state.
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

impl<T> Interactive for Portal<T>
where
    T: StateData,
{
    type State = T;

    fn interact(&mut self, _state: &mut ResMut<WorldState>) -> Vec<Action<Self::State>> {
        Action::Transition(self.state.clone()).single()
    }
}
