use bevy::prelude::*;

use crate::camera::{
    next::NextPlugin,
    spot::CameraSpotPlugin,
};

pub struct CameraPlugin;

#[rustfmt::skip]
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(CameraSpotPlugin)
            .add_plugin(NextPlugin);
    }
}
