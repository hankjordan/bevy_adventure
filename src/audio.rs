use std::collections::HashMap;

use bevy::{
    audio::AudioSource,
    ecs::system::SystemParam,
    prelude::*,
};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app ////
            .init_resource::<AudioRegistry>()
            .add_systems(Update, cleanup_audio);
    }
}

#[derive(Component)]
struct AudioPlayer;

#[allow(clippy::needless_pass_by_value)]
fn cleanup_audio(mut commands: Commands, sinks: Query<(Entity, &AudioSink), With<AudioPlayer>>) {
    for (entity, sink) in &sinks {
        if sink.empty() {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct AudioRegistry {
    map: HashMap<String, Handle<AudioSource>>,
}

impl AudioRegistry {
    fn insert(&mut self, name: &str, handle: Handle<AudioSource>) {
        self.map.insert(name.to_owned(), handle);
    }

    fn get(&self, name: &str) -> Option<Handle<AudioSource>> {
        if let Some(source) = self.map.get(&name.to_owned()) {
            Some(source.clone())
        } else {
            warn!("Could not find AudioSource with name {:?}", name);
            None
        }
    }
}

/// `SystemParam` for registering `AudioSource` clips by path.
#[derive(SystemParam)]
pub struct AudioServer<'w, 's> {
    commands: Commands<'w, 's>,

    asset_server: Res<'w, AssetServer>,
    registry: ResMut<'w, AudioRegistry>,
}

impl<'w, 's> AudioServer<'w, 's> {
    /// Load an `AudioSource` by path.
    pub fn load(&mut self, name: &str) -> &mut Self {
        self.registry
            .insert(name, self.asset_server.load(name.to_owned()));
        self
    }

    /// Play an `AudioSource` by path.
    pub fn play(&mut self, name: &str) {
        if let Some(source) = self.registry.get(name) {
            self.commands.spawn((
                Name::from(name),
                AudioBundle {
                    source,
                    ..default()
                },
                AudioPlayer,
            ));
        }
    }
}
