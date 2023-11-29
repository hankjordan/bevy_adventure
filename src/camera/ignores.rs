use bevy::prelude::*;

/// A component for a `CameraSpot` that defines what objects are ignored when at that spot.
///
/// Useful for creating area triggers.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Ignores {
    /// The names of the entities to ignore.
    pub names: Vec<String>,
}

impl Ignores {
    /// Returns a new instance of `Ignores`.
    pub fn new(names: Vec<&str>) -> Self {
        Self {
            names: names.into_iter().map(|n| n.to_owned()).collect(),
        }
    }

    /// Returns a new instance of `Ignores`, from a single value.
    pub fn single(name: &str) -> Self {
        Self::new(vec![name])
    }
}
