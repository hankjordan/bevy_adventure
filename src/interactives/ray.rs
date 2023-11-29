use bevy::{
    math::Vec3A,
    prelude::*,
};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Ray3d {
    origin: Vec3A,
    direction: Vec3A,
}

impl Ray3d {
    /// Constructs a `Ray3d`, normalizing the direction vector.
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray3d {
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
            .map(Ray3d::from)
    }
}

impl From<Ray> for Ray3d {
    fn from(ray: Ray) -> Self {
        Ray3d::new(ray.origin, ray.direction)
    }
}
