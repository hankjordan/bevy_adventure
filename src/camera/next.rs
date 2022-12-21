use bevy::prelude::*;

use crate::{animation::Tween, camera::spot::{CurrentSpot, CameraSpot}};

pub struct NextPlugin;

#[rustfmt::skip]
impl Plugin for NextPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(NextSpot::default())
            .add_system_to_stage(CoreStage::PostUpdate, handle_next_spot);
    }
}

/// Resource for setting the next spot the Camera will go to.
#[derive(Resource, Default)]
pub struct NextSpot(pub Option<CameraSpot>);

impl NextSpot {
    /// Returns the current next spot if any, leaving the resource with None.
    pub fn pop(&mut self) -> Option<CameraSpot> {
        self.0.take()
    }

    /// Set the `NextSpot` to the given `CameraSpot`.
    pub fn set(&mut self, spot: CameraSpot) {
        self.0 = Some(spot);
    }

    /// Returns true if `NextSpot` is None.
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
}

fn handle_next_spot(
    mut at_spot: ResMut<CurrentSpot>,
    mut next_spot: ResMut<NextSpot>,
    mut cameras: Query<&mut Tween<Transform>, With<Camera>>,
) {
    if let Some(spot) = next_spot.pop() {
        if let Ok(mut animation) = cameras.get_single_mut() {
            animation.target = spot.transform();
            at_spot.set(spot);
        }
    }
}
