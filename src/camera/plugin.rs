use bevy::prelude::*;

use crate::{
    camera::{
        next::NextPlugin,
        spot::CameraSpotPlugin,
    },
    BackToSpot,
    Ignores,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app ////
            .register_type::<BackToSpot>()
            .register_type::<Ignores>()
            .add_plugins((CameraSpotPlugin, NextPlugin));
    }
}
