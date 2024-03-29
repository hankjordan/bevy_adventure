use bevy::{
    ecs::system::SystemParam,
    prelude::*,
};

pub struct CameraSpotPlugin;

impl Plugin for CameraSpotPlugin {
    fn build(&self, app: &mut App) {
        app ////
            .register_type::<CurrentSpot>()
            .register_type::<Option<CameraSpot>>()
            .register_type::<CameraSpot>()
            .register_type::<IsCameraSpot>()
            ////
            .init_resource::<CurrentSpot>();
    }
}

/// Resource that specifies what `CameraSpot` the player is at.
///
/// If you want to change what position the Camera is currently at, use `NextSpot`.
#[derive(Resource, Default, Debug, Reflect)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[reflect(Resource)]
pub struct CurrentSpot {
    spot: Option<CameraSpot>,
}

impl CurrentSpot {
    /// Create a new `CurrentSpot` from the given `CameraSpot`.
    pub fn new(spot: CameraSpot) -> Self {
        Self { spot: Some(spot) }
    }

    /// Returns the `CameraSpot` the `CurrentSpot` is currently set to.
    ///
    /// # Panics
    /// If the Scene has not been set up yet, or there is no `MAIN_CAMERA` spot.
    pub fn get(&self) -> &CameraSpot {
        self.spot.as_ref().unwrap()
    }

    /// Returns the name of the `CameraSpot` the `CurrentSpot` is currently set to.
    ///
    /// # Panics
    /// If the Scene has not been set up yet, or there is no `MAIN_CAMERA` spot.
    pub fn name(&self) -> &str {
        self.get().name()
    }

    /// Returns true if the `CurrentSpot` is set.
    pub fn is_some(&self) -> bool {
        self.spot.is_some()
    }
}

/// A `CameraSpot` - a location the Camera might be at in the scene.
#[derive(Debug, Reflect)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CameraSpot {
    name: String,
    entity: Entity,
    transform: Transform,
}

impl CameraSpot {
    /// Returns a new `CameraSpot`, given a name, entity, and `Transform`.
    pub fn new(name: &str, entity: Entity, transform: Transform) -> Self {
        Self {
            name: name.to_owned(),
            entity,
            transform,
        }
    }

    /// Returns the name of the `CameraSpot`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the entity associated with the `CameraSpot`.
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// Sets the entity of the `CameraSpot`.
    pub fn set_entity(&mut self, entity: Entity) {
        self.entity = entity;
    }

    /// Returns the transform of the `CameraSpot`.
    pub fn transform(&self) -> Transform {
        self.transform
    }
}

#[derive(Component, Debug, Default, Reflect)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[reflect(Component)]
pub struct IsCameraSpot;

/// `SystemParam` for retrieving `CameraSpots` for entities or from names.
#[derive(SystemParam)]
pub struct CameraSpots<'w, 's> {
    named: Query<'w, 's, (Entity, &'static Name), Without<IsCameraSpot>>,
    spots: Query<'w, 's, (Entity, &'static Name, &'static GlobalTransform), With<IsCameraSpot>>,
}

impl<'w, 's> CameraSpots<'w, 's> {
    /// Given a spot's name, retrieve the associated `CameraSpot`.
    pub fn get(&self, spot: &str) -> Option<CameraSpot> {
        for (entity, spot_name, gtf) in &self.spots {
            if spot == spot_name.as_str() {
                return Some(CameraSpot::new(spot_name, entity, gtf.compute_transform()));
            }
        }

        None
    }

    /// Given an interactive entity, retrieve a `CameraSpot` from the interactive's name.
    ///
    /// Only works if the name of the `CameraSpot` matches `Camera_ENTITY_NAME`
    pub fn for_interactive(&self, entity: Entity) -> Option<CameraSpot> {
        if let Ok((_, name)) = self.named.get(entity) {
            return self.get(format! {"Camera_{name}"}.as_str());
        }

        None
    }

    /// Given a `CameraSpot`, retrieve the associated `Interactive` entity.
    pub fn for_spot(&self, spot: &CameraSpot) -> Option<Entity> {
        if let Some((_, target)) = spot.name().split_once("Camera_") {
            for (entity, name) in &self.named {
                if name.as_str() == target {
                    return Some(entity);
                }
            }
        }

        None
    }
}
