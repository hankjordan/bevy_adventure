use bevy::prelude::*;

use crate::{
    animation::Tween,
    camera::spot::CurrentSpot,
    scene::SceneManager,
    CameraSpots,
};

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
pub struct NextSpot {
    name: Option<String>,
    entity: Option<Entity>,
    jump: bool,
}

impl NextSpot {
    /// Create a new instance of `NextSpot`, which will skip animating the camera when applied.
    ///
    /// If you want a smooth animation, modify the `NextSpot` resource directly instead of inserting a new one.
    pub fn new(name: &str) -> Self {
        Self {
            name: Some(name.to_owned()),
            entity: None,
            jump: true,
        }
    }

    /// Returns the next spot if any, leaving the resource with None.
    pub fn pop(&mut self) -> (Option<String>, Option<Entity>, bool) {
        let jump = self.jump;
        self.jump = false;

        (self.name.take(), self.entity.take(), jump)
    }

    /// Set the `NextSpot` to the given name.
    pub fn set(&mut self, name: &str) {
        self.name = Some(name.to_owned());
    }

    /// Set an entity override for the `NextSpot`.
    pub fn set_entity(&mut self, entity: Entity) {
        self.entity = Some(entity);
    }

    /// Returns true if `NextSpot` is None.
    pub fn is_none(&self) -> bool {
        self.name.is_none()
    }
}

#[allow(clippy::needless_pass_by_value)]
fn handle_next_spot(
    mut commands: Commands,
    scene: SceneManager,
    spots: CameraSpots,
    mut next_spot: ResMut<NextSpot>,
    mut cameras: Query<(&mut Transform, &mut Tween<Transform>), With<Camera>>,
) {
    if !scene.ready() {
        return;
    }

    if let (Some(name), entity_opt, jump) = next_spot.pop() {
        if let Some(mut spot) = spots.get(&name) {
            if let Some(entity) = entity_opt {
                spot.set_entity(entity);
            }

            if let Ok((mut tf, mut animation)) = cameras.get_single_mut() {
                if jump {
                    *tf = spot.transform();
                }

                animation.target = spot.transform();
                commands.insert_resource(CurrentSpot::new(spot));
            }
        } else {
            warn!("Could not find CameraSpot with name {:?}", name);
        }
    }
}
