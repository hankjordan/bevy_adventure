use std::marker::PhantomData;

use bevy::{
    ecs::schedule::StateData,
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
where S: StateData {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Hovering>()
            .init_resource::<Interaction>()

            .add_system(interactive::<Simple<NoState>>)
            .add_system(interactive::<Simple<S>>)
            
            .add_system(interactive::<Prop>)
            .add_system(interactive::<Trigger>)
            
            .add_system_to_stage(CoreStage::PreUpdate, hovering_raycast);
    }
}
