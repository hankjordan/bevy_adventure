use bevy::{
    ecs::{
        schedule::States,
        system::{
            EntityCommands,
            SystemParam,
        },
    },
    prelude::*,
};

use crate::{
    Action,
    AnimationServer,
    AudioServer,
    Simple,
};

/// `SystemParam` that acts as an extension to `Commands` for working with named entities.
#[derive(SystemParam)]
pub struct CommandsExt<'w, 's> {
    commands: Commands<'w, 's>,

    animation_server: AnimationServer<'w, 's>,
    audio_server: AudioServer<'w, 's>,

    query: Query<'w, 's, (Entity, &'static Name)>,
    visibility: Query<'w, 's, &'static mut Visibility>,
}

impl<'w, 's> CommandsExt<'w, 's> {
    /// Returns a Vec of entities matching the given name.
    pub fn named(&self, target: &str) -> Vec<Entity> {
        let mut result = Vec::new();

        for (entity, name) in &self.query {
            if name.as_str() == target {
                result.push(entity);
            }
        }

        result
    }

    /// Returns a Vec of entities matching any of the given names.
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

    /// Despawn all entities with the given name.
    pub fn despawn_named(&mut self, target: &str) {
        for entity in self.named(target) {
            self.commands.entity(entity).despawn_recursive();
        }
    }

    /// Despawn all entities with any of the given names.
    pub fn despawn_all_named(&mut self, targets: &Vec<&str>) {
        for entity in self.named_any(targets) {
            self.commands.entity(entity).despawn_recursive();
        }
    }

    /// Set `Visibility` to `Visiblity::Visible` for all entities with the given name.
    pub fn show_named(&mut self, target: &str) {
        for entity in self.named(target) {
            if let Ok(mut visibility) = self.visibility.get_mut(entity) {
                *visibility = Visibility::Visible;
            }
        }
    }

    /// Set `Visibility` to `Visibility::Hidden` for all entities with the given name.
    pub fn hide_named(&mut self, target: &str) {
        for entity in self.named(target) {
            if let Ok(mut visibility) = self.visibility.get_mut(entity) {
                *visibility = Visibility::Hidden;
            }
        }
    }

    /// Play a named animation on the [`AnimationServer`]
    pub fn play_animation(&mut self, name: &str) {
        self.animation_server.play(name);
    }

    /// Play a named audio clip on the [`AudioServer`]
    pub fn play_audio(&self, name: &str) {
        self.audio_server.play(name);
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

/// Extension trait that adds Action-related methods to Bevy's `EntityCommands`.
pub trait CommandsActionsExt {
    /// Insert a `SimpleInteractive` `Component` that runs the given actions when interacted with.
    fn actions<State: States>(&mut self, actions: Vec<Action<State>>) -> &mut Self;
}

impl<'w, 's, 'a> CommandsActionsExt for EntityCommands<'w, 's, 'a> {
    fn actions<State: States>(&mut self, actions: Vec<Action<State>>) -> &mut Self {
        self.insert(Simple::from(actions))
    }
}
