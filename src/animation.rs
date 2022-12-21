use std::{collections::HashMap, marker::PhantomData};

use bevy::{prelude::*, ecs::system::SystemParam};

use crate::Scene;

pub struct AnimationPlugin;

#[rustfmt::skip]
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AnimationRegistry::default())
            
            .add_system(tween_transforms);
    }
}

#[derive(Component)]
pub struct Tween<T> {
    pub target: T,
}

impl<T> Tween<T> {
    pub fn new(target: T) -> Self {
        Self { target }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn tween_transforms(time: Res<Time>, mut targets: Query<(&Tween<Transform>, &mut Transform)>) {
    let dt = time.delta_seconds();

    for (animation, mut tf) in &mut targets {
        tf.translation = tf.translation.lerp(animation.target.translation, 3.0 * dt);
        tf.rotation = tf.rotation.lerp(animation.target.rotation, 3.0 * dt);
    }
}

#[derive(Resource, Debug, Default)]
pub struct AnimationRegistry {
    map: HashMap<String, Handle<AnimationClip>>,
}

impl AnimationRegistry {
    fn insert(&mut self, name: &str, handle: Handle<AnimationClip>) {
        self.map.insert(
            name.to_owned(),
            handle
        );
    }

    pub fn get(&self, name: &str) -> Option<Handle<AnimationClip>> {
        self.map.get(&name.to_owned()).cloned()
    }
}

/// `SystemParam` for registering named Animations.
#[derive(SystemParam)]
pub struct AnimationServer<'w, 's> {
    asset_server: Res<'w, AssetServer>,
    registry: ResMut<'w, AnimationRegistry>,

    #[system_param(ignore)]
    marker: PhantomData<&'s ()>,
}

impl<'w, 's> AnimationServer<'w, 's> {
    /// Load a named animation for a given Scene.
    pub fn load<S: Scene>(&mut self, name: &str) -> &mut Self {
        let full = S::scene();

        let scene;

        if let Some((short, _)) = full.rsplit_once('#') {
            scene = short;
        } else {
            scene = full;
        }

        let handle = self.asset_server.load(format!("{scene}#{name}"));
        self.registry.insert(name, handle);

        self
    }
}
