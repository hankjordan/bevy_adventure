use std::marker::PhantomData;

use bevy::{
    ecs::schedule::StateData,
    prelude::*,
};

use crate::{
    animation::AnimationPlugin,
    audio::AudioPlugin,
    camera::CameraPlugin,
    cursor::CursorPlugin,
    interactives::InteractivesPlugin,
    inventory::InventoryPlugin,
    scene::SceneManagerPlugin,
    state::WorldStatePlugin,
    textdisplay::TextDisplayPlugin,
};

/// The main plugin that must be added to your app with `add_plugin`.
///
/// The generic parameter `State` should be the `StateData` for your game,
/// usually an enum containing all of the different possible states.
pub struct AdventurePlugin<State>(PhantomData<State>);

impl<State> Default for AdventurePlugin<State> {
    fn default() -> Self {
        Self(PhantomData::default())
    }
}

impl<State> Plugin for AdventurePlugin<State>
where
    State: StateData,
{
    fn build(&self, app: &mut App) {
        app ////
            .add_plugin(AnimationPlugin)
            .add_plugin(AudioPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(CursorPlugin)
            .add_plugin(InteractivesPlugin::<State>::default())
            .add_plugin(InventoryPlugin)
            .add_plugin(SceneManagerPlugin)
            .add_plugin(TextDisplayPlugin)
            .add_plugin(WorldStatePlugin);
    }
}
