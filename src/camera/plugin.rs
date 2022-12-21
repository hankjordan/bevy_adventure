use bevy::prelude::*;

use crate::camera::next::NextPlugin;

pub struct CameraPlugin;

#[rustfmt::skip]
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(NextPlugin);
    }
}
