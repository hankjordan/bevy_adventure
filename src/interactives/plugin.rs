use std::marker::PhantomData;

use bevy::{
    ecs::schedule::States,
    prelude::*,
};

use crate::interactives::{
    hovering::{
        hovering_raycast,
        Hovering,
    },
    interactive,
    simple::{
        NoState,
        Prop,
        Simple,
        Trigger,
    },
    Interaction,
};

pub struct InteractivesPlugin<S>(PhantomData<S>);

impl<S> Default for InteractivesPlugin<S> {
    fn default() -> Self {
        Self(PhantomData::default())
    }
}

#[rustfmt::skip]
impl<S> Plugin for InteractivesPlugin<S> 
where S: States {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Hovering>()
            .init_resource::<Interaction>()

            .add_system(interactive::<Simple<NoState>>)
            .add_system(interactive::<Simple<S>>)
            
            .add_system(interactive::<Prop>)
            .add_system(interactive::<Trigger>)
            
            .add_system(hovering_raycast.in_base_set(CoreSet::PreUpdate));
    }
}
