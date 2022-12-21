#![doc = include_str!("../README.md")]

use bevy::{
    ecs::{
        schedule::StateData,
        system::EntityCommands,
        world::EntityRef,
    },
    prelude::*,
};
use iyes_loopless::prelude::*;

pub use crate::{
    camera::Ignores,
    interactives::{
        interactive,
        Action,
        Interactive,
        MoveTo,
        Portal,
    },
    scene::SceneManager,
    state::WorldState,
};

mod animation;
mod camera;
mod commands;
mod interactives;
mod inventory;
mod scene;
mod state;
mod textdisplay;

#[allow(unused_variables)]
pub trait Scene {
    type State: StateData;

    fn state() -> Self::State;
    fn scene<'a>() -> &'a str;
    fn setup(app: &mut App) {}
    fn spawn(entity: &EntityRef, commands: &mut EntityCommands);
}

pub trait AppSceneStateExt {
    fn add_scene<S: Scene + 'static>(&mut self) -> &mut App;
    fn register_interactive<S, I>(&mut self) -> &mut App
    where
        S: Scene + 'static,
        I: Interactive + Component;
}

impl AppSceneStateExt for App {
    fn add_scene<S: Scene + 'static>(&mut self) -> &mut App {
        S::setup(self);

        self //
            .add_enter_system(S::state(), load_scene::<S>)
            .add_exit_system(S::state(), cleanup_scenes)
    }

    fn register_interactive<S, I>(&mut self) -> &mut App
    where
        S: Scene + 'static,
        I: Interactive + Component,
    {
        self.add_system_set(
            ConditionSet::new()
                .run_in_state(S::state())
                .with_system(interactive::<I>)
                .into(),
        )
    }
}

fn load_scene<S: Scene + 'static>(mut manager: SceneManager) {
    manager.spawn(S::scene(), S::spawn)
}

fn cleanup_scenes(
    mut commands: Commands,
    scenes: Query<Entity, With<Handle<bevy::prelude::Scene>>>,
) {
    for scene in &scenes {
        commands.entity(scene).despawn_recursive();
    }
}
