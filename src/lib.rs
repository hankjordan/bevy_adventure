#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![doc = include_str!("../README.md")]

pub use crate::{
    animation::AnimationServer,
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

mod animation;
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
