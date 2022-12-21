use bevy::prelude::*;

/// A component for a `CameraSpot` that defines what objects are ignored when at that spot.
/// 
/// Useful for creating area triggers.
#[derive(Component, Default)]
pub struct Ignores {
    pub names: Vec<String>,
}

impl Ignores {
    pub fn new(names: Vec<&str>) -> Self {
        Self {
            names: names.into_iter().map(|n| n.to_owned()).collect(),
        }
    }

    pub fn single(name: &str) -> Self {
        Self::new(vec![name])
    }
}