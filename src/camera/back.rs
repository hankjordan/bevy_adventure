use bevy::prelude::*;

/// A component for a `CameraSpot` that defines what spot you will go back when at that spot.
///
/// Overrides `BackToState`.
#[derive(Component, Debug)]
pub struct BackToSpot {
    /// The name of the target spot.
    pub name: String,
}

impl BackToSpot {
    /// Returns a new instance of `BackToSpot`.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

/// A component for a `CameraSpot` that defines what state you will go back when at that spot.
#[derive(Component, Debug)]
pub struct BackToState<T> {
    /// The state to go back to.
    pub state: T,
}

impl<T> BackToState<T> {
    /// Returns a new instance of `BackToState`.
    pub fn new(state: T) -> Self {
        Self { state }
    }
}
