use bevy::prelude::*;

/// A component for a `CameraSpot` that defines what spot you will go back when at that spot.
///
/// Overrides `BackToState`.
#[derive(Component, Debug)]
pub struct BackToSpot {
    pub spot: String,
}

impl BackToSpot {
    pub fn new(spot: &str) -> Self {
        Self {
            spot: spot.to_owned(),
        }
    }
}

/// A component for a `CameraSpot` that defines what state you will go back when at that spot.
#[derive(Component, Debug)]
pub struct BackToState<T> {
    pub state: T,
}

impl<T> BackToState<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}
