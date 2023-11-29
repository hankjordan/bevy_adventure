use bevy::{
    ecs::schedule::States,
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum NoState {
    #[default]
    State,
}

/// An `Interactive` that just runs the given actions when interacted with.
#[derive(Component)]
pub struct Simple<State> {
    actions: Vec<Action<State>>,
}

impl<State> Simple<State> {
    /// Add an `Action` to the `Simple`.
    pub fn push(&mut self, action: Action<State>) -> &mut Self {
        self.actions.push(action);
        self
    }

    /// Add a `Vec` of Actions to the `Simple`.
    pub fn extend(&mut self, actions: Vec<Action<State>>) -> &mut Self {
        self.actions.extend(actions);
        self
    }

    /// Add an `Action::Animation` action to the `Simple`.
    #[must_use]
    pub fn animation(mut self, name: &str) -> Self {
        self.push(Action::Animation(name.to_owned()));
        self
    }

    /// Add an `Action::Audio` action to the `Simple`.
    #[must_use]
    pub fn audio(mut self, name: &str) -> Self {
        self.push(Action::Audio(name.to_owned()));
        self
    }

    /// Add an `Action::Jump` action to the `Simple`.
    #[must_use]
    pub fn jump(mut self, name: &str) -> Self {
        self.push(Action::Jump(name.to_owned()));
        self
    }
}

impl<State> From<Vec<Action<State>>> for Simple<State> {
    fn from(actions: Vec<Action<State>>) -> Self {
        Self { actions }
    }
}

impl<State> From<Action<State>> for Simple<State> {
    fn from(action: Action<State>) -> Self {
        <Vec<Action<State>>>::from(action).into()
    }
}

impl<State> Interactive for Simple<State>
where
    State: States,
{
    type State = State;

    fn interact(&mut self, _: &mut ResMut<WorldState>) -> Vec<Action<Self::State>> {
        self.actions.clone()
    }
}

/// A preset `Interactive` that displays a message when interacted with.
pub struct Description;

impl Description {
    /// Returns a new instance of `Simple` that will send a message with the given text when interacted with.
    pub fn build(text: &str) -> Simple<NoState> {
        Action::Message(Message::new(text)).into()
    }
}

/// A preset `Interactive` that moves to a `CameraSpot` when interacted with.
pub struct MoveTo;

impl MoveTo {
    /// Returns a new instance of `Simple` that will move the camera to the given spot when interacted with.
    pub fn build(spot: &str) -> Simple<NoState> {
        Action::Move(spot.to_owned()).into()
    }
}

/// A preset `Interactive` that changes the current state when interacted with.
pub struct Portal;

impl Portal {
    /// Returns a new instance of `Simple` that will change the current state when interacted with.
    pub fn build<State>(state: State) -> Simple<State> {
        Action::Transition(state).into()
    }
}

/// A preset `Interactive` that does nothing when interacted with.
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

/// A preset `Interactive` that does nothing when interacted with.
///
/// Useful for creating camera triggers, prevents interacting with objects behind it until it is focused.
#[derive(Component)]
pub struct Trigger;

impl Trigger {
    /// Create a new `Bundle`, with `Ignores` set up to ignore the passed name, and the `Trigger` `Interactive`.
    pub fn build(name: &str) -> (Ignores, Self) {
        (Ignores::single(name), Self)
    }
}

impl Interactive for Trigger {
    type State = NoState;

    fn interact(&mut self, _state: &mut ResMut<WorldState>) -> Vec<Action<Self::State>> {
        vec![]
    }
}
