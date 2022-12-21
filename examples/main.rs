use bevy::{
    ecs::{
        system::EntityCommands,
        world::EntityRef,
    },
    prelude::*,
};
use bevy_adventure::{
    Action,
    AdventurePlugin,
    AppSceneStateExt,
    Ignores,
    Interactive,
    Message,
    MoveTo,
    NewMessage,
    Portal,
    Scene,
    WorldState,
};
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    Bathroom,
    Hallway,
}

#[derive(Component)]
struct Cup;

impl Interactive for Cup {
    type State = GameState;

    fn interact(&mut self, _state: &mut ResMut<WorldState>) -> Vec<Action<Self::State>> {
        Action::Message(Message::new("It's a cup.")).single()
    }
}

struct BathroomScene;

impl Scene for BathroomScene {
    type State = GameState;

    fn state() -> Self::State {
        GameState::Bathroom
    }

    fn scene<'a>() -> &'a str {
        "scenes/bathroom.glb#Scene0"
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

struct HallwayScene;

impl Scene for HallwayScene {
    type State = GameState;

    fn state() -> Self::State {
        GameState::Hallway
    }

    fn scene<'a>() -> &'a str {
        "scenes/hallway.glb#Scene0"
    }

    fn spawn(entity: &EntityRef, commands: &mut EntityCommands) {
        const BATHROOM_DOOR: &str = "Door.001";

        match entity.get::<Name>().map(|t| t.as_str()) {
            Some(BATHROOM_DOOR) => commands
                .insert(Collider::cuboid(0.1, 0.5, 1.1))
                .insert(Portal::new(GameState::Bathroom)),
            _ => commands,
        };
    }
}

fn display_messages(mut messages: EventReader<NewMessage>) {
    for message in messages.iter() {
        println!("Message: {:?}", message);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().set(AssetPlugin {
            asset_folder: "examples/assets".to_owned(),
            ..default()
        }))
        ////
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        ////
        .add_plugin(AdventurePlugin::<GameState>::default())
        ////
        .add_loopless_state(GameState::Bathroom)
        ////
        .add_scene::<BathroomScene>()
        .add_scene::<HallwayScene>()
        ////
        .add_system(display_messages)
        ////
        .run();
}
