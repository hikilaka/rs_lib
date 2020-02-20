#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Item {
    identifier: usize,
    quantity: usize,
}

impl Item {
    pub fn new(identifier: usize, quantity: usize) -> Self {
        Item {
            identifier: identifier,
            quantity: quantity,
        }
    }
    pub fn identifier(&self) -> usize {
        self.identifier
    }
    pub fn quantity(&self) -> usize {
        self.quantity
    }
}
