use std::marker::PhantomData;

use bevy::{
    state::prelude::States,
    prelude::*,
    state::state::FreelyMutableState, // Add import
};

use crate::{
    interactives::{
        hovering::HoveringPlugin,
        interact::InteractionPlugin,
        interactive,
    },
    scene::{AdventureScene, AppSceneStateExt},
    Interactive,
    SceneRoot,
};

pub struct InteractivesPlugin<S> {
    _marker: std::marker::PhantomData<S>,
}

impl<S> Default for InteractivesPlugin<S> {
    fn default() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<S> Plugin for InteractivesPlugin<S>
where
    S: States + FreelyMutableState + 'static,
{
    fn build(&self, app: &mut App) {
        use crate::interactives::simple::{NoState, Prop, Simple, Trigger};
        
        app.add_plugins((
            crate::interactives::hovering::HoveringPlugin,
            crate::interactives::interact::InteractionPlugin,
        ))
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
