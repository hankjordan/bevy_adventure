use bevy::{
    ecs::{
        system::EntityCommands,
        world::EntityRef,
    },
    prelude::*,
};
use bevy_adventure::{
    Action,
    AppSceneStateExt,
    Interactive,
    Scene,
    WorldState, Portal, MoveTo, Ignores,
};

use bevy_rapier3d::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    Bathroom,
    Hallway
}

#[derive(Component)]
struct Cup;

impl Interactive for Cup {
    type State = GameState;

    fn interact(&mut self, state: &mut ResMut<WorldState>) -> Vec<Action<Self::State>> {
        todo!()
    }
}

struct BathroomScene;

impl Scene for BathroomScene {
    type State = GameState;

    fn state() -> Self::State {
        GameState::Bathroom
    }

    fn scene<'a>() -> &'a str {
        "scenes/c1/bathroom.glb#Scene0"
    }

    fn setup(app: &mut App) {
        app ////
            .register_interactive::<Self, Cup>();
    }

    fn spawn(entity: &EntityRef, commands: &mut EntityCommands) {
        const CUP: &str = "Cup";
        const DOOR: &str = "Door";
        const SINK: &str = "Sink";
        const SINK_CAMERA: &str = "Camera_Sink";

        match entity.get::<Name>().map(|t| t.as_str()) {
            Some(SINK_CAMERA) => commands.insert(Ignores::single(SINK)),
            Some(CUP) => commands.insert(Collider::cuboid(0.1, 0.1, 0.1)).insert(Cup),
            Some(DOOR) => commands
                .insert(Collider::cuboid(0.1, 0.5, 1.1))
                .insert(Portal::new(GameState::Hallway)),
            Some(SINK) => commands
                .insert(Collider::cuboid(0.5, 0.5, 0.7))
                .insert(MoveTo::new(SINK_CAMERA)),
            
            _ => commands,
        };
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        ////
        .add_scene::<BathroomScene>()
        ////
        .run();
}
