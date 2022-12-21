use bevy::{
    ecs::system::SystemParam,
    prelude::*,
};

#[derive(SystemParam)]
pub struct CommandsExt<'w, 's> {
    commands: Commands<'w, 's>,
    query: Query<'w, 's, (Entity, &'static Name)>,
    visibility: Query<'w, 's, &'static mut Visibility>,
}

impl<'w, 's> CommandsExt<'w, 's> {
    pub fn named(&self, target: &str) -> Vec<Entity> {
        let mut result = Vec::new();

        for (entity, name) in &self.query {
            if name.as_str() == target {
                result.push(entity);
            }
        }

        result
    }

    pub fn named_any<T: AsRef<str>>(&self, targets: &Vec<T>) -> Vec<Entity> {
        let mut result = Vec::new();

        'outer: for (entity, name) in &self.query {
            for target in targets {
                if name.as_str() == target.as_ref() {
                    result.push(entity);
                    continue 'outer;
                }
            }
        }

        result
    }

    pub fn despawn_named(&mut self, target: &str) {
        for entity in self.named(target) {
            self.commands.entity(entity).despawn_recursive();
        }
    }

    pub fn despawn_all_named(&mut self, targets: &Vec<&str>) {
        for entity in self.named_any(targets) {
            self.commands.entity(entity).despawn_recursive();
        }
    }

    pub fn show_named(&mut self, target: &str) {
        for entity in self.named(target) {
            if let Ok(mut visibility) = self.visibility.get_mut(entity) {
                visibility.is_visible = true;
            }
        }
    }

    pub fn hide_named(&mut self, target: &str) {
        for entity in self.named(target) {
            if let Ok(mut visibility) = self.visibility.get_mut(entity) {
                visibility.is_visible = false;
            }
        }
    }
}

impl<'w, 's> std::ops::Deref for CommandsExt<'w, 's> {
    type Target = Commands<'w, 's>;

    fn deref(&self) -> &Self::Target {
        &self.commands
    }
}

impl<'w, 's> std::ops::DerefMut for CommandsExt<'w, 's> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.commands
    }
}
