use std::collections::{
    HashMap,
    HashSet,
};

use bevy::prelude::*;
#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use crate::textdisplay::{
    Message,
    TextDisplay,
};

pub struct InventoryPlugin;

#[rustfmt::skip]
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DraggingItem::default())
            .insert_resource(Inventory::default())
            .insert_resource(Recipes::default())
            
            .add_system(handle_combine);
    }
}

/// A resource that stores the player's current inventory.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Resource, Debug, Default)]
pub struct Inventory {
    /// The items held in the inventory.
    pub items: HashSet<String>,
}

/// A resource that stores which items are being dragged, if any.
#[derive(Resource, Default)]
pub struct DraggingItem {
    /// Source item name
    pub src: Option<String>,

    /// Destination item name
    pub dst: Option<String>,
}

impl DraggingItem {
    /// Returns true if either the source or the destination is Some.
    pub fn is_dragging(&self) -> bool {
        self.src.is_some() || self.dst.is_some()
    }
}

/// A resource that stores all registered item combinations.
#[derive(Resource, Default)]
pub struct Recipes {
    map: HashMap<(String, String), String>,
}

impl Recipes {
    /// Insert a new recipe into the map.
    /// Order is ignored (`(a, b) == (b, a)`).
    pub fn insert(&mut self, a: &str, b: &str, result: &str) {
        self.map
            .insert((a.to_owned(), b.to_owned()), result.to_owned());
        self.map
            .insert((b.to_owned(), a.to_owned()), result.to_owned());
    }

    /// Given a source and a destination, return the matching combination result, if any.
    pub fn get(&self, src: &str, dst: &str) -> Option<&String> {
        self.map.get(&(src.to_owned(), dst.to_owned()))
    }
}

#[allow(clippy::needless_pass_by_value)]
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

                let src = src.clone();
                let dst = dst.clone();
                let result = result.clone();

                display.show(Message::ItemCombine { src, dst, result });
            } else {
                display.show(Message::InvalidItemCombination);
            }
        }
    }
}
