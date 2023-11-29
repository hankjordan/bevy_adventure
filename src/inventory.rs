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

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app ////
            .init_resource::<DraggingItem>()
            .init_resource::<Inventory>()
            .init_resource::<Recipes>()
            ////
            .add_systems(Update, handle_combine);
    }
}

/// A resource that stores the player's current inventory.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Resource, Debug, Default)]
pub struct Inventory {
    /// The items held in the inventory.
    pub items: HashSet<Item>,
}

/// An item.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Item {
    name: String,
}

impl Item {
    /// Returns a new Item with the given name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }

    /// The name of the Item.
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl From<&str> for Item {
    fn from(name: &str) -> Self {
        Item::new(name)
    }
}

impl From<String> for Item {
    fn from(name: String) -> Self {
        Item::new(&name)
    }
}

/// A resource that stores which items are being dragged, if any.
#[derive(Resource, Default)]
pub struct DraggingItem {
    /// Source item name
    pub src: Option<Item>,

    /// Destination item name
    pub dst: Option<Item>,
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
    map: HashMap<(Item, Item), Item>,
}

impl Recipes {
    /// Insert a new recipe into the map.
    /// Order is ignored (`(a, b) == (b, a)`).
    pub fn insert(&mut self, a: &str, b: &str, result: &str) {
        self.map.insert((a.into(), b.into()), result.into());
        self.map.insert((b.into(), a.into()), result.into());
    }

    /// Given a source and a destination, return the matching combination result, if any.
    pub fn get(&self, src: &Item, dst: &Item) -> Option<&Item> {
        self.map.get(&(src.clone(), dst.clone()))
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
