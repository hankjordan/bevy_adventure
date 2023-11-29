use bevy::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app ////
            .init_resource::<Cursor>()
            .add_systems(First, update_cursor);
    }
}

/// A resource that stores the current position of the Cursor.
///
/// Updated with the current mouse position whenever it is moved.
/// This resource can be used to render a cursor icon or move the cursor from gamepad inputs.
/// The position of this Cursor resource is used when interacting with objects in the scene.
#[derive(Resource, Default)]
pub struct Cursor {
    position: Vec2,
    last_position: Vec2,
}

impl Cursor {
    /// Set the current cursor position.
    pub fn set(&mut self, position: Vec2) {
        self.last_position = self.position;
        self.position = position;
    }

    /// Change the current cursor position by the given offset.
    pub fn offset(&mut self, offset: Vec2) {
        self.set(self.position + offset);
    }

    /// Returns the current cursor position.
    pub fn position(&self) -> Vec2 {
        self.position
    }
}

#[allow(clippy::needless_pass_by_value)]
fn update_cursor(mut cursor: ResMut<Cursor>, windows: Query<&Window>) {
    for window in &windows {
        if let Some(mouse) = window.cursor_position() {
            if mouse != cursor.last_position {
                cursor.set(mouse);
            }
        }
    }
}
