use bevy::prelude::*;

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

#[derive(Component, Debug)]
pub struct BackToState<T> {
    pub state: T,
}

impl<T> BackToState<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}
