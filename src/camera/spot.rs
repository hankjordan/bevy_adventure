use bevy::{
    ecs::system::SystemParam,
    prelude::*,
};

#[derive(Component, Debug)]
pub struct IsCameraSpot;

/// A component for a Camera that specifies what `CameraSpot` it is at.
#[derive(Component, Debug)]
pub struct AtSpot(pub Entity);

#[derive(Debug)]
pub struct CameraSpot {
    pub entity: Entity,
    pub transform: Transform,
}

impl CameraSpot {
    pub fn new(entity: Entity, gtf: &GlobalTransform) -> Self {
        Self { entity, transform: gtf.compute_transform() }
    }
}

#[derive(SystemParam)]
pub struct CameraSpots<'w, 's> {
    named: Query<'w, 's, &'static Name, Without<IsCameraSpot>>,
    spots: Query<'w, 's, (Entity, &'static Name, &'static GlobalTransform), With<IsCameraSpot>>,
}

impl<'w, 's> CameraSpots<'w, 's> {
    pub fn get(&self, spot: &str) -> Option<CameraSpot> {
        for (entity, spot_name, gtf) in &self.spots {
            if spot == spot_name.as_str() {
                return Some(CameraSpot::new(entity, gtf));
            }
        }

        None
    }

    pub fn find(&self, entity: Entity) -> Option<CameraSpot> {
        if let Ok(name) = self.named.get(entity) {
            return self.get(format! {"Camera_{name}"}.as_str());
        }

        None
    }
}
