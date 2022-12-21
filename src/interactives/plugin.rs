use std::marker::PhantomData;

use bevy::{prelude::*, ecs::schedule::StateData};

use crate::interactives::{
    interactive,
    simple::*,
};

pub struct InteractivesPlugin<S>(PhantomData<S>);

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
