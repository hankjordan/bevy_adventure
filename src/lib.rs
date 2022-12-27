#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![doc = include_str!("../README.md")]

mod animation;
mod audio;
mod camera;
mod commands;
mod constants;
mod cursor;
mod interactives;
mod inventory;
mod plugin;
mod scene;
mod state;
mod textdisplay;

pub use crate::{
    animation::AnimationServer,
    audio::AudioServer,
    camera::{
        BackToSpot,
        BackToState,
        CameraSpot,
        CameraSpots,
        CurrentSpot,
        Ignores,
        NextSpot,
    },
    commands::CommandsExt,
    constants::MAIN_CAMERA,
    cursor::Cursor,
    interactives::{
        invalid_item_used,
        Action,
        Description,
        Hovering,
        Interactive,
        ItemRef,
        MoveTo,
        Portal,
        Prop,
        Trigger,
    },
    inventory::{
        DraggingItem,
        Inventory,
        Item,
        Recipes,
    },
    plugin::AdventurePlugin,
    scene::{
        AppSceneStateExt,
        Scene,
    },
    state::WorldState,
    textdisplay::{
        Message,
        NewMessage,
    },
};
