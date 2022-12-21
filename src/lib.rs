#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![doc = include_str!("../README.md")]

pub use crate::{
    animation::AnimationServer,
    camera::{
        AtSpot,
        BackToSpot,
        BackToState,
        Ignores,
    },
    commands::CommandsExt,
    constants::MAIN_CAMERA,
    interactives::{
        invalid_item_used,
        Action,
        Description,
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
mod interactives;
mod inventory;
mod plugin;
mod scene;
mod state;
mod textdisplay;
