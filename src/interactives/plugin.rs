use std::marker::PhantomData;

use bevy::{
    ecs::schedule::StateData,
    prelude::*,
};

use crate::interactives::{
    interactive,
    simple::{
        Description,
        MoveTo,
        Portal,
    },
};

pub struct InteractivesPlugin<S>(PhantomData<S>);

impl<S> Default for InteractivesPlugin<S> {
    fn default() -> Self {
        Self(PhantomData::default())
    }
}

#[rustfmt::skip]
impl<S> Plugin for InteractivesPlugin<S> 
where S: StateData {
    fn build(&self, app: &mut App) {
        app
            .add_system(interactive::<Description>)
            .add_system(interactive::<MoveTo>)
            .add_system(interactive::<Portal<S>>);
    }
}
