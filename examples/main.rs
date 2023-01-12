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
    AnimationServer,
    AppSceneStateExt,
    AudioServer,
    CommandsExt,
    Description,
    Interactive,
    Item,
    Message,
    NewMessage,
    Scene,
    Simple,
    Trigger,
    WorldState,
};
use bevy_rapier3d::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    Bathroom,
    Bedroom,
    Hallway,
}

// Bathroom |----------------------------------------------------------------------------------------------------------

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
            .add_interactive::<Self, Cup>();
    }

    fn spawn(entity: &EntityRef, commands: &mut EntityCommands) {
        const CUP: &str = "Cup";
        const DOOR: &str = "Door";
        const SINK: &str = "Sink";

        match entity.get::<Name>().map(|t| t.as_str()) {
            Some(CUP) => commands.insert(Collider::cuboid(0.1, 0.1, 0.1)).insert(Cup),
            Some(DOOR) => commands.insert(Door::build(GameState::Hallway)),

            // Creating a Trigger will make the Interactive act as though it isn't there, but only when focused.
            // In this scene, focusing on the sink allows the player to interact with the Cup.
            Some(SINK) => commands
                .insert(Collider::cuboid(0.5, 0.5, 0.7))
                .insert(Trigger::build(SINK)),

            _ => commands,
        };
    }
}

// Bedroom |-----------------------------------------------------------------------------------------------------------

#[derive(Component, Default)]
struct Dresser {
    next: usize,
}

const DRESSER_TAKEN: &str = "bedroom_dresser_taken";

const DRESSER_TOP_OPEN: &str = "Animation3";
const DRESSER_TOP_CLOSE: &str = "Animation2";
const DRESSER_BOTTOM_OPEN: &str = "Animation1";
const DRESSER_BOTTOM_CLOSE: &str = "Animation0";

const DRESSER_SFX_OPEN: &str = "sfx/drawer_open.ogg";
const DRESSER_SFX_CLOSE: &str = "sfx/drawer_close.ogg";

const FLASHLIGHT_OBJECT: &str = "Flashlight";

const ITEM_FLASHLIGHT_EMPTY: &str = "Flashlight (empty)";

impl Interactive for Dresser {
    type State = GameState;

    fn update(&mut self, commands: &mut CommandsExt, state: &mut ResMut<WorldState>) {
        if state.get_bool(DRESSER_TAKEN) {
            commands.despawn_named(FLASHLIGHT_OBJECT);
        }
    }

    fn interact(&mut self, state: &mut ResMut<WorldState>) -> Vec<Action<Self::State>> {
        let actions: Vec<Vec<Action<Self::State>>> = vec![
            vec![
                Action::Animation(DRESSER_BOTTOM_OPEN.to_owned()),
                Action::Audio(DRESSER_SFX_OPEN.to_owned()),
            ],
            vec![
                Action::Animation(DRESSER_BOTTOM_CLOSE.to_owned()),
                Action::Audio(DRESSER_SFX_CLOSE.to_owned()),
            ],
            vec![
                Action::Animation(DRESSER_TOP_OPEN.to_owned()),
                Action::Audio(DRESSER_SFX_OPEN.to_owned()),
            ],
            vec![
                Item::new(ITEM_FLASHLIGHT_EMPTY).into(),
                Action::Audio(SFX_ITEM_PICKUP.to_owned()),
            ],
            vec![
                Action::Animation(DRESSER_TOP_CLOSE.to_owned()),
                Action::Audio(DRESSER_SFX_CLOSE.to_owned()),
            ],
        ];

        if self.next == 3 {
            if state.get_bool(DRESSER_TAKEN) {
                self.next += 1;
            } else {
                state.set(DRESSER_TAKEN, true);
            }
        }

        let val = actions[self.next].clone();

        self.next = (self.next + 1) % actions.len();

        val
    }
}

#[derive(Component, Default)]
struct TrashCan;

const ITEM_BATTERIES: &str = "Batteries";
const TRASH_CAN_EMPTY: &str = "bedroom_trash_can_empty";

impl Interactive for TrashCan {
    type State = GameState;

    fn update(&mut self, commands: &mut CommandsExt, state: &mut ResMut<WorldState>) {
        if state.get_bool(TRASH_CAN_EMPTY) {
            commands.despawn_all_named(&vec!["Battery_01", "Battery_02"]);
        }
    }

    fn interact(&mut self, state: &mut ResMut<WorldState>) -> Vec<Action<Self::State>> {
        if state.get_bool(TRASH_CAN_EMPTY) {
            Action::Message(Message::new("The trash can is empty.")).single()
        } else {
            state.set(TRASH_CAN_EMPTY, true);

            vec![
                Item::new(ITEM_BATTERIES).into(),
                Action::Audio(SFX_ITEM_PICKUP.to_owned()),
            ]
        }
    }
}

struct BedroomScene;

impl Scene for BedroomScene {
    type State = GameState;

    fn state() -> Self::State {
        GameState::Bedroom
    }

    fn scene<'a>() -> &'a str {
        "scenes/bedroom.glb#Scene0"
    }

    fn animations(server: &mut AnimationServer) {
        server ////
            .load::<Self>(DRESSER_TOP_OPEN)
            .load::<Self>(DRESSER_TOP_CLOSE)
            .load::<Self>(DRESSER_BOTTOM_OPEN)
            .load::<Self>(DRESSER_BOTTOM_CLOSE);
    }

    fn audio(server: &mut AudioServer) {
        server ////
            .load(DRESSER_SFX_OPEN)
            .load(DRESSER_SFX_CLOSE);
    }

    fn setup(app: &mut App) {
        app ////
            .add_interactive::<Self, Dresser>()
            .add_interactive::<Self, TrashCan>();
    }

    fn spawn(entity: &EntityRef, commands: &mut EntityCommands) {
        const BED: &str = "Bed";
        const DOOR: &str = "Door";
        const DOOR_HINGE_TOP: &str = "Door_Hinge_Top";
        const DRESSER: &str = "Dresser";
        const TRASH_CAN: &str = "Trash_Can";
        const PAINTING: &str = "Painting";
        const OLD_MONITOR: &str = "Old_Monitor";
        const HAMPER: &str = "Hamper";
        const BOOKSHELF: &str = "Bookshelf";

        match entity.get::<Name>().map(|t| t.as_str()) {
            Some(BED) => commands
                .insert(Collider::cuboid(0.5, 1.0, 0.5))
                .insert(Description::build("It's a bed.")),
            Some(DOOR_HINGE_TOP) => commands
                .insert(Collider::cuboid(0.25, 0.25, 0.25))
                .insert(Description::build("It's a door hinge.")),
            Some(DOOR) => commands.insert(Door::build(GameState::Hallway)),
            Some(DRESSER) => commands
                .insert(Collider::cuboid(0.75, 0.5, 0.5))
                .insert(Dresser::default()),

            // Note that we don't actually define a CameraSpot for the Trash Can
            // but the camera still zooms in on it when interacting.
            //
            // This is because the CameraSpot with the name `Camera_Trash_Can`
            // is automatically tried when we interact with the Trash Can.
            //
            // Creating a camera in the scene with the name Camera_OBJECT will
            // cause this effect for OBJECT.
            Some(TRASH_CAN) => commands
                .insert(Collider::cuboid(0.3, 0.3, 0.3))
                .insert(TrashCan),

            Some(PAINTING) => commands
                .insert(Collider::cuboid(0.5, 0.5, 0.5))
                .insert(Description::build("I like this painting.")),
            Some(OLD_MONITOR) => {
                commands
                    .insert(Collider::cuboid(0.3, 0.3, 0.3))
                    .insert(Description::build(
                        "My computer barely works. I really need a new one.",
                    ))
            }
            Some(HAMPER) => commands
                .insert(Collider::cuboid(0.3, 0.3, 0.3))
                .insert(Description::build("The hamper is empty.")),
            Some(BOOKSHELF) => {
                commands
                    .insert(Collider::cuboid(0.5, 0.2, 0.5))
                    .insert(Description::build(
                        "The books are ordered and lined up neatly.",
                    ))
            }

            _ => commands,
        };
    }
}

// Hallway |-----------------------------------------------------------------------------------------------------------

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
        const BEDROOM_DOOR: &str = "Door";
        const BATHROOM_DOOR: &str = "Door.001";

        match entity.get::<Name>().map(|t| t.as_str()) {
            Some(BATHROOM_DOOR) => commands.insert(Door::build(GameState::Bathroom)),
            Some(BEDROOM_DOOR) => commands.insert(Door::build(GameState::Bedroom)),
            _ => commands,
        };
    }
}

// Builders |----------------------------------------------------------------------------------------------------------

// This builder sets up a Bundle with a door-sized Collider, a Transition Action that switches scenes, and an Audio Action that plays a sound.
pub struct Door;

impl Door {
    pub fn build<State>(state: State) -> (Collider, Simple<State>) {
        (
            Collider::cuboid(0.1, 0.5, 1.1),
            vec![
                Action::Transition(state),
                Action::Audio(SFX_DOOR_ENTER.to_owned()),
            ]
            .into(),
        )
    }
}

// Setup |-------------------------------------------------------------------------------------------------------------

const SFX_ITEM_PICKUP: &str = "sfx/pickup.ogg";
const SFX_DOOR_ENTER: &str = "sfx/door.ogg";

fn setup_audio(mut server: AudioServer) {
    server ////
        .load(SFX_ITEM_PICKUP)
        .load(SFX_DOOR_ENTER);
}

fn print_messages(mut messages: EventReader<NewMessage>) {
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
        // Important: register state via `add_adventure_state` instead of `add_loopless_state`
        // This allows NextSpot to work between Scenes (Requires `SystemTransitionStage` to run after `CoreStage::Update`)
        .add_adventure_state(GameState::Bathroom)
        ////
        .add_scene::<BathroomScene>()
        .add_scene::<BedroomScene>()
        .add_scene::<HallwayScene>()
        ////
        .add_startup_system(setup_audio)
        ////
        .add_system(print_messages)
        ////
        .run();
}
