use std::collections::HashMap;

use bevy::{
    ecs::system::SystemParam,
    prelude::*,
};

use crate::Scene;

pub struct AnimationPlugin;

#[rustfmt::skip]
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AnimationRegistry::default())
            .insert_resource(AnimationQueue::default())
            
            .add_system(tween_transforms)
            .add_system(play_animations);
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
        self.map.insert(name.to_owned(), handle);
    }

    fn get(&self, name: &str) -> Option<Handle<AnimationClip>> {
        if let Some(animation) = self.map.get(&name.to_owned()) {
            Some(animation.clone())
        } else {
            warn!("Could not find AnimationClip with name {:?}", name);
            None
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct AnimationQueue {
    queued: HashMap<Entity, Vec<Handle<AnimationClip>>>,
    playing: HashMap<Entity, Handle<AnimationClip>>,
}

impl AnimationQueue {
    pub fn push(&mut self, entity: Entity, handle: Handle<AnimationClip>) {
        let entry = self.queued.entry(entity).or_default();
        entry.push(handle);
    }

    pub fn pop(&mut self, entity: Entity) -> Option<Handle<AnimationClip>> {
        if let Some(entry) = self.queued.get_mut(&entity) {
            if let Some(handle) = entry.pop() {
                return Some(handle);
            } else {
                self.queued.remove(&entity);
            }
        }

        None
    }

    pub fn keys(&self) -> impl Iterator<Item = Entity> + '_ {
        self.queued.keys().map(|i| *i)
    }
}

fn play_animations(
    clips: Res<Assets<AnimationClip>>,
    mut queue: ResMut<AnimationQueue>,
    mut players: Query<&mut AnimationPlayer>,
) {
    let mut remove = Vec::new();

    for (entity, handle) in queue.playing.iter() {
        if let Ok(player) = players.get(*entity) {
            if let Some(clip) = clips.get(handle) {
                if player.elapsed() > clip.duration() {
                    remove.push(*entity);
                }
            } else {
                remove.push(*entity);
            }
        } else {
            remove.push(*entity);
        }
    }

    for entity in remove {
        queue.playing.remove(&entity);
    }

    let targets = queue.keys().collect::<Vec<_>>();

    for entity in targets {
        if let Ok(mut player) = players.get_mut(entity) {
            if !queue.playing.contains_key(&entity) {
                if let Some(next) = queue.pop(entity) {
                    queue.playing.insert(entity, next.clone());
                    player.play(next);
                }
            }
        }
    }
}

/// `SystemParam` for registering named Animations.
#[derive(SystemParam)]
pub struct AnimationServer<'w, 's> {
    asset_server: Res<'w, AssetServer>,
    assets: Res<'w, Assets<AnimationClip>>,
    registry: ResMut<'w, AnimationRegistry>,

    queue: ResMut<'w, AnimationQueue>,

    players: Query<'w, 's, (Entity, &'static Name), With<AnimationPlayer>>,
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

    /// Returns a handle to a loaded `AnimationClip` given a name.
    pub fn get(&self, name: &str) -> Option<Handle<AnimationClip>> {
        self.registry.get(name)
    }

    /// Play an animation with the associated `AnimationPlayer`
    pub fn play(&mut self, name: &str) {
        if let Some(handle) = self.get(name) {
            if let Some(animation) = self.assets.get(&handle) {
                let mut names = Vec::new();

                for (path, _) in animation.curves() {
                    names.extend(&path.parts);
                }

                for (entity, name) in &self.players {
                    if names.contains(&name) {
                        self.queue.push(entity, handle);
                        break;
                    }
                }
            }
        }
    }
}
