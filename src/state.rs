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
    /// Retrieve a value from the map for a given key.
    ///
    /// The output is parsed from the stored string.
    pub fn get<T: FromStr>(&self, key: &str) -> Option<T> {
        self.map
            .get(&key.to_owned())
            .and_then(|v| v.parse::<T>().ok())
    }

    /// Retrieve a bool value from the map for a given key.
    ///
    /// Returns false if the key does not exist.
    pub fn get_bool(&self, key: &str) -> bool {
        self.get(key).unwrap_or_default()
    }

    /// Insert a value into the map.
    ///
    /// The value is converted to a string, so it must implement `ToString`.
    #[allow(clippy::needless_pass_by_value)]
    pub fn insert<T: ToString>(&mut self, key: &str, value: T) {
        self.map.insert(key.to_owned(), value.to_string());
    }

    /// Alias for insert.
    pub fn set<T: ToString>(&mut self, key: &str, value: T) {
        self.insert(key, value);
    }
}
