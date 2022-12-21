pub struct ItemRef<'a> {
    item: &'a String,
    consumed: bool,
}

impl<'a> ItemRef<'a> {
    pub fn new(item: &'a String) -> Self {
        Self {
            item,
            consumed: false,
        }
    }

    pub fn consume(&mut self) {
        self.consumed = true;
    }

    pub fn consumed(&self) -> bool {
        self.consumed
    }

    pub fn as_str(&self) -> &str {
        self.item.as_str()
    }
}