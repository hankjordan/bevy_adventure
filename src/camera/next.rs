use bevy::prelude::*;

use crate::{animation::Tween, camera::spot::{AtSpot, CameraSpot}};

pub struct NextPlugin;

#[rustfmt::skip]
impl Plugin for NextPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(NextSpot::default())
            .add_system_to_stage(CoreStage::PostUpdate, handle_next_spot);
    }
}

#[derive(Resource, Default)]
pub struct NextSpot(pub Option<CameraSpot>);

impl NextSpot {
    pub fn pop(&mut self) -> Option<CameraSpot> {
        self.0.take()
    }

    pub fn set(&mut self, spot: CameraSpot) {
        self.0 = Some(spot);
    }
    
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
}

fn handle_next_spot(
    mut next_spot: ResMut<NextSpot>,
    mut cameras: Query<(&mut AtSpot, &mut Tween<Transform>), With<Camera>>,
) {
    if let Some(spot) = next_spot.pop() {
        if let Ok((mut at_spot, mut animation)) = cameras.get_single_mut() {
            at_spot.0 = spot.entity;
            animation.target = spot.transform;
        }
    }
}
