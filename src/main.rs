mod collections;
mod entity;

use collections::{Container, Inventory};
use entity::Item;

fn main() {
    let mut inv = Inventory::with_capacity(5);

    println!("{:?}", inv);

    let _ = inv.add_at(Item::new(10, 1), 2);

    println!("{:?}", inv);
}
