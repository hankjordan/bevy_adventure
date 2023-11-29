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
        Self(PhantomData)
    }
}

impl<S> Plugin for InteractivesPlugin<S>
where
    S: States,
{
    fn build(&self, app: &mut App) {
        app ////
            .init_resource::<Hovering>()
            .init_resource::<Interaction>()
            ////
            .add_systems(
                Update,
                (
                    interactive::<Simple<NoState>>,
                    interactive::<Simple<S>>,
                    interactive::<Prop>,
                    interactive::<Trigger>,
                ),
            )
            .add_systems(PreUpdate, hovering_raycast);
    }
}
