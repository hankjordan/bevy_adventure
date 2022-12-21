use bevy::{
    asset::AssetPath,
    ecs::{
        system::{
            EntityCommands,
            SystemParam,
        },
        world::EntityRef,
    },
    prelude::*,
    scene::SceneInstance,
};

use crate::camera::IsCameraSpot;

pub struct SceneManagerPlugin;

#[rustfmt::skip]
impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(run_hooks);
    }
}

#[derive(Component, Debug)]
pub struct SceneHooked;

#[allow(clippy::type_complexity)]
#[derive(Component)]
pub struct SceneHook {
    hook: Box<dyn Fn(&EntityRef, &mut EntityCommands) + Send + Sync + 'static>,
}
impl SceneHook {
    pub fn new<F: Fn(&EntityRef, &mut EntityCommands) + Send + Sync + 'static>(hook: F) -> Self {
        Self {
            hook: Box::new(hook),
        }
    }
}

fn run_hooks(
    world: &World,
    mut commands: Commands,
    spawner: Res<SceneSpawner>,
    unloaded: Query<(Entity, &SceneInstance, &SceneHook), Without<SceneHooked>>,
) {
    for (entity, instance, hooked) in unloaded.iter() {
        if spawner.instance_is_ready(**instance) {
            commands.entity(entity).insert(SceneHooked);
        }
        let entities = spawner.iter_instance_entities(**instance);
        for entity_ref in entities.filter_map(|e| world.get_entity(e)) {
            let mut cmd = commands.entity(entity_ref.id());
            (hooked.hook)(&entity_ref, &mut cmd);
        }
    }
}

#[derive(SystemParam)]
pub struct SceneManager<'w, 's> {
    commands: Commands<'w, 's>,
    asset_server: Res<'w, AssetServer>,
}

impl<'w, 's> SceneManager<'w, 's> {
    pub fn load<'a, P, F>(&mut self, path: P, hook: F) -> impl Bundle
    where
        P: Into<AssetPath<'a>>,
        F: Fn(&EntityRef, &mut EntityCommands) + Send + Sync + 'static,
    {
        (
            SceneBundle {
                scene: self.asset_server.load(path),
                ..default()
            },
            SceneHook::new(move |entity, commands| {
                hook(entity, commands);

                if entity.contains::<Camera>() {
                    commands.remove::<Camera>();
                    commands.insert(IsCameraSpot);
                }
            }),
        )
    }

    pub fn spawn<'a, P, F>(&mut self, path: P, hook: F)
    where
        P: Into<AssetPath<'a>>,
        F: Fn(&EntityRef, &mut EntityCommands) + Send + Sync + 'static,
    {
        let bundle = self.load(path, hook);
        self.commands.spawn(bundle);
    }
}
