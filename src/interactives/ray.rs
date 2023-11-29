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

    pub fn from_screenspace(
        cursor_pos_screen: Vec2,
        camera: &Camera,
        camera_transform: &GlobalTransform,
    ) -> Option<Self> {
        let view = camera_transform.compute_matrix();

        let viewport = camera.logical_viewport_rect()?;
        let (viewport_min, viewport_max) = (viewport.min, viewport.max);
        
        let screen_size = camera.logical_target_size()?;
        let viewport_size = viewport_max - viewport_min;
        let adj_cursor_pos =
            cursor_pos_screen - Vec2::new(viewport_min.x, screen_size.y - viewport_max.y);

        let projection = camera.projection_matrix();
        let far_ndc = projection.project_point3(Vec3::NEG_Z).z;
        let near_ndc = projection.project_point3(Vec3::Z).z;
        let cursor_ndc = (adj_cursor_pos / viewport_size) * 2.0 - Vec2::ONE;
        let ndc_to_world: Mat4 = view * projection.inverse();
        let near = ndc_to_world.project_point3(cursor_ndc.extend(near_ndc));
        let far = ndc_to_world.project_point3(cursor_ndc.extend(far_ndc));
        let ray_direction = far - near;
        Some(Ray3d::new(near, ray_direction))
    }
}
