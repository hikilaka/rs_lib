use super::{Container, ContainerError, ContainerResult};
use crate::entity::Item;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum InventorySlot {
    Empty,
    Item(Item),
}

/// Provides a default implementation of a container.
#[derive(Debug, Clone)]
pub struct Inventory {
    capacity: usize,
    item_count: usize,
    items: Vec<InventorySlot>,
}

impl Container<Item> for Inventory {
    fn with_capacity(capacity: usize) -> Self {
        let mut inv = Inventory {
            capacity: capacity,
            item_count: 0,
            items: Vec::new(),
        };

        inv.items.resize(capacity, InventorySlot::Empty);

        inv
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn count(&self) -> usize {
        self.item_count
    }

    fn contains(&self, item: &Item) -> bool {
        self.items.contains(&InventorySlot::Item(item.clone()))
    }

    fn add(&mut self, item: Item) -> ContainerResult<()> {
        // TODO check stackability
        for slot in self.items.iter_mut() {
            if *slot == InventorySlot::Empty {
                *slot = InventorySlot::Item(item);
                self.item_count += 1;
                return Ok(());
            }
        }
        Err(ContainerError::Full)
    }

    fn add_at(&mut self, item: Item, slot: usize) -> ContainerResult<()> {
        if slot >= self.capacity {
            return Err(ContainerError::IndexOutOfBounds);
        }

        self.items[slot] = InventorySlot::Item(item);
        self.item_count += 1;
        Ok(())
    }

    fn remove(&mut self, item: &Item) -> ContainerResult<()> {
        for slot in self.items.iter_mut() {
            if let InventorySlot::Item(i) = slot {
                if i.identifier() == item.identifier() {
                    if i.quantity() > item.quantity() {
                        return Err(ContainerError::QuantityInsufficient);
                    }

                    let difference = item.quantity() - i.quantity();

                    if difference == 0 {
                        *slot = InventorySlot::Empty;
                        self.item_count -= 1;
                    } else {
                        let new_item = Item::new(i.identifier(), difference);
                        *slot = InventorySlot::Item(new_item);
                    }
                    return Ok(());
                }
            }
        }
        Err(ContainerError::NotFound)
    }

    fn remove_at(&mut self, slot: usize) -> ContainerResult<()> {
        if slot >= self.capacity {
            return Err(ContainerError::IndexOutOfBounds);
        }

        if let InventorySlot::Item(_) = self.items[slot] {
            self.items[slot] = InventorySlot::Empty;
            self.item_count -= 1;
            Ok(())
        } else {
            Err(ContainerError::NotFound)
        }
    }

    fn get_at(&self, slot: usize) -> ContainerResult<Item> {
        if slot >= self.capacity {
            return Err(ContainerError::IndexOutOfBounds);
        }

        if let InventorySlot::Item(item) = &self.items[slot] {
            Ok(item.clone())
        } else {
            Err(ContainerError::NotFound)
        }
    }

    fn swap(&mut self, slot_a: usize, slot_b: usize) -> ContainerResult<()> {
        if slot_a >= self.capacity || slot_b >= self.capacity {
            return Err(ContainerError::IndexOutOfBounds);
        }

        self.items.swap(slot_a, slot_b);
        Ok(())
    }
}

#[cfg(test)]
mod inventory_tests {
    use super::{Container, ContainerError, Inventory, Item};

    #[test]
    fn inv_capacity_and_add() {
        let mut inv = Inventory::with_capacity(5);

        assert_eq!(inv.capacity(), 5);
        for i in 0..inv.capacity() {
            assert_eq!(inv.count(), i);
            assert_eq!(inv.add(Item::new(i, 1)), Ok(()));
            assert_eq!(inv.count(), i + 1);
        }

        assert_eq!(inv.add(Item::new(6, 1)), Err(ContainerError::Full));
    }

    #[test]
    fn inv_add_remove_at() {
        let mut inv = Inventory::with_capacity(5);

        for i in 0..5 {
            assert_eq!(inv.count(), i);
            assert_eq!(inv.add_at(Item::new(i, 1), i), Ok(()));
            assert_eq!(inv.count(), i + 1);
        }

        assert_eq!(
            inv.add_at(Item::new(6, 1), 10),
            Err(ContainerError::IndexOutOfBounds)
        );

        for j in 0..5 {
            assert_eq!(inv.count(), 4 - j + 1);
            assert_eq!(inv.remove_at(4 - j), Ok(()));
            assert_eq!(inv.count(), 4 - j);
        }

        assert_eq!(inv.remove_at(1), Err(ContainerError::NotFound));
    }

    #[test]
    fn inv_swap() {
        let mut inv = Inventory::with_capacity(3);

        let _ = inv.add_at(Item::new(0, 1), 0);
        // 1st index left empty
        let _ = inv.add_at(Item::new(2, 1), 2);

        assert_eq!(inv.swap(0, 2), Ok(()));

        // check that the 0th index contains item #2
        assert_eq!(inv.get_at(0).unwrap().identifier(), 2);
        // check that the 2nd index contains item #0
        assert_eq!(inv.get_at(2).unwrap().identifier(), 0);

        assert_eq!(inv.swap(0, 1), Ok(()));

        // check that the 0th index contains nothing
        assert_eq!(inv.get_at(0), Err(ContainerError::NotFound));
        // check that the 1st index contains item #2
        assert_eq!(inv.get_at(1).unwrap().identifier(), 2);

        assert_eq!(inv.swap(0, 50), Err(ContainerError::IndexOutOfBounds));
    }
}
