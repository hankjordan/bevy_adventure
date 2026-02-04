use bevy::{
    math::Vec3A,
    prelude::*,
};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct AdventureRay {
    origin: Vec3A,
    direction: Vec3A,
}

impl AdventureRay {
    /// Constructs a `AdventureRay`, normalizing the direction vector.
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        AdventureRay {
            origin: origin.into(),
            direction: direction.normalize().into(),
        }
    }

    /// Position vector describing the ray origin
    pub fn origin(&self) -> Vec3 {
        self.origin.into()
    }

    /// Unit vector describing the ray direction
    pub fn direction(&self) -> Vec3 {
        self.direction.into()
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn from_screenspace(
        cursor_pos_screen: Vec2,
        camera: &Camera,
        camera_transform: &GlobalTransform,
        window: &Window,
    ) -> Option<Self> {
        let mut viewport_pos = cursor_pos_screen;
        if let Some(viewport) = &camera.viewport {
            viewport_pos -= viewport.physical_position.as_vec2() / window.scale_factor() as f32;
        }
        camera
            .viewport_to_world(camera_transform, viewport_pos)
            .ok()
            .map(|r| AdventureRay::from(r))
    }
}

impl From<Ray3d> for AdventureRay {
    fn from(ray: Ray3d) -> Self {
        AdventureRay::new(ray.origin, *ray.direction)
    }
}
