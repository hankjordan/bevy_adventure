use bevy::prelude::*;
use bevy::state::prelude::*;
use bevy::state::state::FreelyMutableState;
use bevy::scene::SceneInstance;
use bevy::ecs::system::{SystemParam, EntityCommands};
use bevy::asset::AssetPath;

use crate::{
    animation::{AnimationServer, Tween},
    audio::AudioServer,
    camera::{CameraSpot, IsCameraSpot},
    commands::CommandsActionsExt,
    interactives::{
        Interactives,
        Hovering,
        prepare_interaction,
        reset_interaction,
        interactive,
    },
    inventory::Inventory,
    state::WorldState,
    textdisplay::TextDisplay,
    CurrentSpot,
    Interactive,
    MAIN_CAMERA,
    SceneHooked, SceneRoot,
};

pub struct SceneManagerPlugin;

impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app ////
            .add_systems(Update, run_hooks);
    }
}


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

#[allow(clippy::needless_pass_by_value)]
fn run_hooks(
    world: &World,
    mut commands: Commands,
    spawner: Res<SceneSpawner>,
    unloaded: Query<(Entity, &SceneInstance, &SceneHook), Without<SceneHooked>>,
) {
    for (entity, instance, hooked) in unloaded.iter() {
        if spawner.instance_is_ready(**instance) {
            commands.entity(entity).insert(SceneHooked);
            for entity_id in spawner.iter_instance_entities(**instance) {
                if let Ok(entity_ref) = world.get_entity(entity_id) {
                    let mut cmd = commands.entity(entity_id);
                    (hooked.hook)(&entity_ref, &mut cmd);
                }
            }
        }
    }
}

#[derive(SystemParam)]
pub struct SceneManager<'w, 's> {
    commands: Commands<'w, 's>,
    asset_server: Res<'w, AssetServer>,
    cameras: Query<'w, 's, Entity, With<Camera>>,
    unloaded: Query<'w, 's, Entity, (With<SceneHook>, Without<SceneHooked>)>,
}

impl<'w, 's> SceneManager<'w, 's> {
    pub fn ready(&self) -> bool {
        self.unloaded.is_empty()
    }

    pub fn load<'a, P, F>(&mut self, path: P, hook: F) -> impl Bundle
    where
        P: Into<AssetPath<'a>>,
        F: Fn(&EntityRef, &mut EntityCommands) + Send + Sync + 'static,
    {
        let camera;

        if let Ok(res) = self.cameras.single() {
            camera = res;
        } else {
            camera = self.commands.spawn(()).id();
        }

        (
            SceneRoot(self.asset_server.load(path)),
            SceneHook::new(move |entity, commands| {
                if entity.contains::<Camera>() {
                    if let Some(name) = entity.get::<Name>() {
                        if name.as_str() == MAIN_CAMERA {
                            let tf = entity.get::<Transform>().unwrap();

                            let spot = CameraSpot::new(name, entity.id(), *tf);
                            let spot_tf = spot.transform();
                            commands.commands().insert_resource(CurrentSpot::new(spot));

                            // The original `camera` variable is used here.
                            // The provided edit had a nested `if name.as_str() == MAIN_CAMERA`
                            // and redefined `camera` inside it, which was redundant and
                            // would have caused `self.cameras.single()` to be called inside the closure,
                            // which is not allowed.
                            commands
                                .commands().entity(camera)
                                .insert(Camera3d::default())
                                .insert(Transform::from(spot_tf))
                                .insert(Tween::new(spot_tf));
                        }
                    }

                    commands.remove::<Camera>();
                    commands.insert(IsCameraSpot);
                }

                if let Some(light) = entity.get::<PointLight>() {
                    let mut light = light.clone(); // Clone the light to modify
                    light.shadows_enabled = true;
                    commands.insert(light);
                }

                hook(entity, commands);
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

/// Trait that provides a `Plugin`-like interface for defining game scenes.
#[allow(unused_variables)]
pub trait AdventureScene {
    /// The type of the state that the scene is a part of.
    type State: States + FreelyMutableState;

    /// The specific state the app will be in when this Scene is active.
    fn state() -> Self::State;

    /// A path to a scene file that can be loaded by Bevy's `asset_loader`.
    fn scene<'a>() -> &'a str;

    /// An optional setup method that allows you to register animations by name.
    fn animations(server: &mut AnimationServer) {}

    /// An optional setup method that allows you to register audio sources by name.
    fn audio(server: &mut AudioServer) {}

    /// An optional setup method that allows you to modify the App when adding the Scene.
    fn setup(app: &mut App) {}

    /// A callback that fires for each entity spawned by loading the Scene.
    fn spawn(entity: &EntityRef, commands: &mut EntityCommands);
}

/// Extension trait that adds Scene-related methods to Bevy's `App`.
pub trait AppSceneStateExt {
    /// Add a Scene to the app.
    ///
    /// Calls the Scene's setup method.
    fn add_scene<S: AdventureScene + 'static>(&mut self) -> &mut App;

    /// Register an interactive for a Scene.
    fn add_interactive<S, I>(&mut self) -> &mut App
    where
        S: AdventureScene + FreelyMutableState + 'static,
        I: Interactive + Component<Mutability = bevy::ecs::component::Mutable>,
        I::State: FreelyMutableState;
}

impl AppSceneStateExt for App {
    fn add_scene<S: AdventureScene + 'static>(&mut self) -> &mut App {
        S::setup(self);

        self ////
            .add_systems(First, reset_interaction)
            .add_systems(PreUpdate, prepare_interaction::<S::State>)
            .add_systems(OnEnter(S::state()), spawn_scene::<S>)
            .add_systems(OnExit(S::state()), cleanup_scene)
    }

    fn add_interactive<S, I>(&mut self) -> &mut App
    where
        S: AdventureScene + FreelyMutableState + 'static,
        I: Interactive + Component<Mutability = bevy::ecs::component::Mutable>,
        I::State: FreelyMutableState,
    {
        self.add_systems(Update, (interactive::<I>).run_if(in_state(S::state())))
    }
}

fn spawn_scene<S: AdventureScene + 'static>(
    mut manager: SceneManager,
    mut animation_server: AnimationServer,
    mut audio_server: AudioServer,
) {
    S::animations(&mut animation_server);
    S::audio(&mut audio_server);
    manager.spawn(S::scene(), S::spawn);
}

#[allow(clippy::needless_pass_by_value)]
fn cleanup_scene(
    mut commands: Commands,
    scenes: Query<Entity, With<SceneRoot>>,
) {
    for scene in &scenes {
        commands.entity(scene).despawn();
    }
}
