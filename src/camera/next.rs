use bevy::{
    ecs::system::SystemState,
    prelude::*,
};

use crate::{
    animation::Tween,
    camera::spot::CurrentSpot,
    scene::SceneManager,
    CameraSpots,
};

pub struct NextPlugin;

impl Plugin for NextPlugin {
    fn build(&self, app: &mut App) {
        app ////
            .add_systems(Last, handle_next_spot);
    }
}

/// Insert this resource at the same time as `NextSpot` to skip animation.
#[derive(Resource, Default)]
pub struct SkipAnimation;

/// Insert this resource to determine what `CameraSpot` the Camera will move to next.
#[derive(Resource, Default)]
pub struct NextSpot(pub String);

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::type_complexity)]
fn handle_next_spot(world: &mut World) {
    let mut state: SystemState<SceneManager> = SystemState::new(world);

    let scene = state.get(world);

    if !scene.ready() {
        return;
    }

    let next = world.remove_resource::<NextSpot>();
    let skip = world.remove_resource::<SkipAnimation>().is_some();

    if let Some(next) = next {
        let mut state: SystemState<(
            CameraSpots,
            Query<(&mut Transform, &mut Tween<Transform>), With<Camera>>,
        )> = SystemState::new(world);

        let (spots, mut cameras) = state.get_mut(world);

        if let Some(spot) = spots.get(&next.0) {
            if let Ok((mut tf, mut animation)) = cameras.get_single_mut() {
                if skip {
                    *tf = spot.transform();
                }

                animation.target = spot.transform();
                world.insert_resource(CurrentSpot::new(spot));
            }
        } else {
            for entity in world.iter_entities().map(|e| e.id()) {
                info!("Entity {:?}", world.inspect_entity(entity));
            }

            warn!("Could not find CameraSpot with name {:?}", next.0);
        }
    }
}
