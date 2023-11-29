use std::marker::PhantomData;

use bevy::{
    ecs::schedule::States,
    prelude::*,
};

use crate::interactives::{
    hovering::HoveringPlugin,
    interact::InteractionPlugin,
    interactive,
    simple::{
        NoState,
        Prop,
        Simple,
        Trigger,
    },
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
            .add_plugins((HoveringPlugin, InteractionPlugin))
            ////
            //.register_type::<Simple<NoState>>()
            //.register_type::<Simple<S>>()
            ////
            ////
            .add_systems(
                Update,
                (
                    interactive::<Simple<NoState>>,
                    interactive::<Simple<S>>,
                    interactive::<Prop>,
                    interactive::<Trigger>,
                ),
            );
    }
}
