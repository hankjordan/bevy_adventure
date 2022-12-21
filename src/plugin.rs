use std::marker::PhantomData;

use bevy::{
    ecs::schedule::StateData,
    prelude::*,
};

use crate::{
    animation::AnimationPlugin,
    camera::CameraPlugin,
    interactives::InteractivesPlugin,
    inventory::InventoryPlugin,
    scene::SceneManagerPlugin,
    state::WorldStatePlugin,
    textdisplay::TextDisplayPlugin,
};

/// The main plugin that must be added to your app with `add_plugin`.
pub struct AdventurePlugin<S>(PhantomData<S>);

impl<S> Default for AdventurePlugin<S> {
    fn default() -> Self {
        Self(PhantomData::default())
    }
}

impl<S> Plugin for AdventurePlugin<S>
where
    S: StateData,
{
    fn build(&self, app: &mut App) {
        app ////
            .add_plugin(AnimationPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(InteractivesPlugin::<S>::default())
            .add_plugin(InventoryPlugin)
            .add_plugin(SceneManagerPlugin)
            .add_plugin(TextDisplayPlugin)
            .add_plugin(WorldStatePlugin);
    }
}
