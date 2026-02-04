use bevy::{
    ecs::system::{SystemParam, SystemState},
    log::prelude::*,
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
            .register_type::<SkipAnimation>()
            .register_type::<NextSpot>()
            ////
            .add_systems(Last, handle_next_spot);
    }
}

/// Insert this resource at the same time as `NextSpot` to skip animation.
#[derive(Resource, Default, Reflect)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[reflect(Resource)]
pub struct SkipAnimation;

/// Insert this resource to determine what `CameraSpot` the Camera will move to next.
#[derive(Resource, Default, Reflect)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[reflect(Resource)]
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
            // Bevy 0.18: single_mut returns (Mut<T>, ...)
            if let Some((mut tf, mut animation)) = cameras.iter_mut().next() {
                 if skip {
                    *tf = spot.transform();
                }

                animation.target = spot.transform();
                world.insert_resource(CurrentSpot::new(spot));
            }
        } else {
            for entity in world.query::<Entity>().iter(&world) {
                info!("Entity {:?}", entity);
            }

            warn!("Could not find CameraSpot with name {:?}", next.0);
        }
    }
}
