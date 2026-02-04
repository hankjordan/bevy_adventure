use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::state::prelude::*;
use bevy::state::state::FreelyMutableState;

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

/// The main plugin that must be added to your app with `App::add_plugin`.
///
/// The generic parameter `S` should be the [`States`] for your game,
/// usually an enum containing all of the different possible states.
pub struct AdventurePlugin<S>(PhantomData<S>);

impl<S> Default for AdventurePlugin<S> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<S> Plugin for AdventurePlugin<S>
where
    S: States + FreelyMutableState,
{
    fn build(&self, app: &mut App) {
        app ////
            .add_plugins((
                AnimationPlugin,
                AudioPlugin,
                CameraPlugin,
                CursorPlugin,
                InteractivesPlugin::<S>::default(),
                InventoryPlugin,
                SceneManagerPlugin,
                TextDisplayPlugin,
                WorldStatePlugin,
            ));
    }
}
