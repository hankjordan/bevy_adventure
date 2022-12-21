use std::collections::HashMap;

use bevy::prelude::*;

pub struct AnimationPlugin;

#[rustfmt::skip]
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AnimationServer::default())
            
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

fn tween_transforms(time: Res<Time>, mut targets: Query<(&Tween<Transform>, &mut Transform)>) {
    let dt = time.delta_seconds();

    for (animation, mut tf) in &mut targets {
        tf.translation = tf.translation.lerp(animation.target.translation, 3.0 * dt);
        tf.rotation = tf.rotation.lerp(animation.target.rotation, 3.0 * dt);
    }
}

#[derive(Resource, Debug, Default)]
pub struct AnimationServer {
    map: HashMap<String, Handle<AnimationClip>>,
}

impl AnimationServer {
    pub fn load(&mut self, asset_server: &Res<AssetServer>, scene: &str, name: &str) {
        self.map.insert(
            name.to_owned(),
            asset_server.load(format!("{scene}#{name}")),
        );
    }

    pub fn get(&self, name: &str) -> Option<Handle<AnimationClip>> {
        self.map.get(&name.to_owned()).cloned()
    }
}
