use std::{
    collections::HashMap,
    str::FromStr,
};

use bevy::prelude::*;
#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

pub struct WorldStatePlugin;

impl Plugin for WorldStatePlugin {
    fn build(&self, app: &mut App) {
        app ////
            .insert_resource(WorldState::default());
    }
}

/// Stringly-typed key value store for tracking game progression.
/// 
/// Use it when you want to persist state for interactives (or anything else that needs to work with interactives).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Resource, Debug, Default)]
pub struct WorldState {
    map: HashMap<String, String>,
}

impl WorldState {
    pub fn get<T: FromStr>(&self, key: &str) -> Option<T> {
        self.map
            .get(&key.to_owned())
            .and_then(|v| v.parse::<T>().ok())
    }

    pub fn get_bool(&self, key: &str) -> bool {
        self.get(key).unwrap_or_default()
    }

    pub fn insert<T: ToString>(&mut self, key: &str, value: &T) {
        self.map.insert(key.to_owned(), value.to_string());
    }

    pub fn set<T: ToString>(&mut self, key: &str, value: &T) {
        self.insert(key, value);
    }
}
