use std::{
    collections::HashMap,
    marker::PhantomData,
};

use bevy::{
    audio::AudioSource,
    ecs::system::SystemParam,
    prelude::*,
};

pub struct AudioPlugin;

#[rustfmt::skip]
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AudioRegistry::default());
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
    asset_server: Res<'w, AssetServer>,
    registry: ResMut<'w, AudioRegistry>,
    //player: Res<'w, Audio>,

    #[system_param(ignore)]
    marker: PhantomData<&'s ()>,
}

impl<'w, 's> AudioServer<'w, 's> {
    /// Load an `AudioSource` by path.
    pub fn load(&mut self, name: &str) -> &mut Self {
        // TODO
        //self.registry.insert(name, self.asset_server.load(name));
        self
    }

    /// Play an `AudioSource` by path.
    pub fn play(&self, name: &str) {
        if let Some(source) = self.registry.get(name) {
            // TODO
            //self.player.play(source);
        }
    }
}
