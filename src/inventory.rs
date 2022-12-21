use std::collections::{
    HashMap,
    HashSet,
};

use bevy::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    interactives::invalid_combine,
    textdisplay::{
        Message,
        TextDisplay,
    },
};

pub struct InventoryPlugin;

#[rustfmt::skip]
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DraggingItem::default())
            .insert_resource(Inventory::default())
            .insert_resource(Recipes::default())

            .add_startup_system(setup_recipes)
            
            .add_system(handle_combine);
    }
}

#[derive(Resource, Debug, Default, Serialize, Deserialize)]
pub struct Inventory {
    pub items: HashSet<String>,
}

#[derive(Clone, Debug)]
pub struct AddedItem {
    pub name: String,
    pub message: Option<String>,
}

impl AddedItem {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            message: None,
        }
    }

    pub fn with_message(mut self, message: &str) -> Self {
        self.message = Some(message.to_owned());
        self
    }
}

#[derive(Resource, Default)]
pub struct DraggingItem {
    pub src: Option<String>,
    pub dst: Option<String>,
}

impl DraggingItem {
    pub fn is_dragging(&self) -> bool {
        self.src.is_some() || self.dst.is_some()
    }
}

#[derive(Resource, Default)]
pub struct Recipes {
    map: HashMap<(String, String), String>,
}

impl Recipes {
    pub fn insert(&mut self, a: &str, b: &str, result: &str) {
        self.map
            .insert((a.to_owned(), b.to_owned()), result.to_owned());
        self.map
            .insert((b.to_owned(), a.to_owned()), result.to_owned());
    }

    pub fn get(&self, src: &str, dst: &str) -> Option<&String> {
        self.map.get(&(src.to_owned(), dst.to_owned()))
    }
}

fn setup_recipes(mut recipes: ResMut<Recipes>) {
    recipes.insert("Flashlight (empty)", "Batteries", "Flashlight");
}

fn handle_combine(
    input: Res<Input<MouseButton>>,
    mut display: TextDisplay,
    mut inventory: ResMut<Inventory>,
    recipes: Res<Recipes>,
    dragging: Res<DraggingItem>,
) {
    if input.just_released(MouseButton::Left) {
        if let (Some(src), Some(dst)) = (&dragging.src, &dragging.dst) {
            if let Some(result) = recipes.get(src, dst) {
                inventory.items.remove(src);
                inventory.items.remove(dst);

                inventory.items.insert(result.clone());

                display.show(Message::new(&format!(
                    "You combine the items to create {result}."
                )));
            } else {
                display.show(invalid_combine());
            }
        }
    }
}
