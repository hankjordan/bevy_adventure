use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    interactives::{
        interact::LookingAt,
        ray::Ray3d,
    },
    CommandsExt,
    CurrentSpot,
    Cursor,
    Ignores,
};

/// The entity that the cursor is currently hovering over, if any.
///
/// This may or may not be an Interactive.
#[derive(Resource, Default)]
pub struct Hovering {
    /// The entity.
    pub entity: Option<Entity>,
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::too_many_arguments)]
pub fn hovering_raycast(
    commands: CommandsExt,

    ctx: Res<RapierContext>,
    cursor: Res<Cursor>,
    at_spot: Res<CurrentSpot>,
    looking_at: Option<Res<LookingAt>>,

    mut hovering: ResMut<Hovering>,

    cameras: Query<(&Camera, &GlobalTransform)>,
    ignore: Query<&Ignores>,
) {
    if let Ok((camera, gtf)) = cameras.get_single() {
        let ray = Ray3d::from_screenspace(cursor.position(), camera, gtf).unwrap();

        let mut ignores = Vec::new();

        if let Some(looking_at) = looking_at {
            if let Ok(ignored) = ignore.get(looking_at.0) {
                ignores.extend(commands.named_any(&ignored.names));
            }
        }

        if let Ok(ignored) = ignore.get(at_spot.get().entity()) {
            ignores.extend(commands.named_any(&ignored.names));
        }

        hovering.entity = None;

        if let Some((entity, _)) = ctx.cast_ray(
            ray.origin(),
            ray.direction(),
            64.0,
            true,
            QueryFilter::new().predicate(&|entity| !ignores.contains(&entity)),
        ) {
            hovering.entity = Some(entity);
        }
    }
}
