use bevy::prelude::*;

use crate::camera::{
    next::NextPlugin,
    spot::CameraSpotPlugin,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app ////
            .add_plugins((CameraSpotPlugin, NextPlugin));
    }
}
