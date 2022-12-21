/// A reference to an item, passed into an Interactive.
/// 
/// Allows the Interactive to optionally consume the item when used.
pub struct ItemRef<'a> {
    item: &'a String,
    consumed: bool,
}

impl<'a> ItemRef<'a> {
    /// Create a new `ItemRef`, given a borrowed item name.
    pub fn new(item: &'a String) -> Self {
        Self {
            item,
            consumed: false,
        }
    }

    /// Consume the item, remove it from the player's inventory.
    pub fn consume(&mut self) {
        self.consumed = true;
    }

    /// Returns whether the item has been consumed or not.
    pub fn consumed(&self) -> bool {
        self.consumed
    }

    /// Returns the item name, as a str.
    pub fn as_str(&self) -> &str {
        self.item.as_str()
    }
}