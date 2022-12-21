use bevy::{prelude::*, ecs::schedule::StateData};

use crate::{
    interactives::{
        Action,
        Interactive,
    },
    state::WorldState, textdisplay::Message,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoState;

#[derive(Component)]
pub struct Description {
    text: String,
}

impl Description {
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

#[derive(Component)]
pub struct MoveTo {
    spot: String,
}

impl MoveTo {
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

#[derive(Component)]
pub struct Portal<T> {
    state: T,
}

impl<T> Portal<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

impl<T> Interactive for Portal<T> 
where T: StateData {
    type State = T;

    fn interact(&mut self, _state: &mut ResMut<WorldState>) -> Vec<Action<Self::State>> {
        Action::Transition(self.state.clone()).single()
    }
}