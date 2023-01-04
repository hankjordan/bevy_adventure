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
    Ignores,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoState;

/// A preset `Interactive` that displays a message when interacted with.
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

/// A preset `Interactive` that moves to a `CameraSpot` when interacted with.
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

/// A preset `Interactive` that changes the current state when interacted with.
#[derive(Component)]
pub struct Portal<T> {
    state: T,
    spot: Option<String>,
}

impl<T> Portal<T> {
    /// Returns a new instance of `Portal` with the given state.
    pub fn new(state: T) -> Self {
        Self { state, spot: None }
    }

    /// Set the `NextSpot` when activating the `Portal`.
    #[must_use]
    pub fn spot(mut self, name: &str) -> Self {
        self.spot = Some(name.to_owned());
        self
    }
}

impl<T> Interactive for Portal<T>
where
    T: StateData,
{
    type State = T;

    fn interact(&mut self, _state: &mut ResMut<WorldState>) -> Vec<Action<Self::State>> {
        let mut actions = Action::Transition(self.state.clone()).single();

        if let Some(spot) = &self.spot {
            actions.push(Action::Jump(spot.clone()));
        }

        actions
    }
}

/// A preset Interactive that does nothing when interacted with.
///
/// Useful for creating objects that are just for looking at.
#[derive(Component)]
pub struct Prop;

impl Interactive for Prop {
    type State = NoState;

    fn interact(&mut self, _state: &mut ResMut<WorldState>) -> Vec<Action<Self::State>> {
        vec![]
    }
}

/// A preset Interactive that does nothing when interacted with.
///
/// Useful for creating camera triggers, prevents interacting with objects behind it until it is focused.
#[derive(Component)]
pub struct Trigger;

impl Trigger {
    /// Create a new bundle, with `Ignores` set up to ignore the passed name, and the `Trigger` `Interactive`.
    pub fn new(name: &str) -> (Ignores, Self) {
        (Ignores::single(name), Self)
    }
}

impl Interactive for Trigger {
    type State = NoState;

    fn interact(&mut self, _state: &mut ResMut<WorldState>) -> Vec<Action<Self::State>> {
        vec![]
    }
}
