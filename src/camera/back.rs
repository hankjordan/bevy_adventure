use bevy::prelude::*;

/// A component for a `CameraSpot` that defines what spot you will go back when at that spot.
///
/// Overrides `BackToState`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Component, Debug, Default)]
pub struct BackToState<S> {
    /// The state to go back to.
    pub state: S,
}

impl<S> BackToState<S> {
    /// Returns a new instance of `BackToState`.
    pub fn new(state: S) -> Self {
        Self { state }
    }
}
