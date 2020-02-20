#[derive(Debug, PartialOrd, PartialEq)]
pub enum ContainerError {
    Full,
    NotFound,
    IndexOutOfBounds,
    QuantityInsufficient,
}

pub type ContainerResult<T> = Result<T, ContainerError>;

pub trait Container<T> {
    /// Creates a new Container with given capacity.
    /// # Example
    /// ```
    /// let inv = Inventory::new(5);
    /// assert_eq!(inv.capacity(), 5);
    /// ```
    fn with_capacity(capacity: usize) -> Self;

    /// Returns the capacity of a container.
    fn capacity(&self) -> usize;

    /// Counts the number of items in a container.
    fn count(&self) -> usize;

    /// Returns whether a container has a specific item or not.
    fn contains(&self, item: &T) -> bool;

    /// Adds a given item to an container.
    ///
    /// # Example
    /// ```
    /// let mut inv = Inventory::with_capacity(1);
    /// assert_eq!(inv.add(Item::new(10)), Ok(()));
    /// assert_eq!(inv.add(Item::new(10)),
    ///            Err(ContainerError::ContainerFull);
    /// ```
    fn add(&mut self, item: T) -> ContainerResult<()>;

    /// Adds an item in a container at a given slot.
    fn add_at(&mut self, item: T, slot: usize) -> ContainerResult<()>;

    /// Removes a given item from a container.
    ///
    /// # Example
    /// ```
    /// let mut inv = Inventory::with_capacity(1);
    /// assert_eq!(inv.add(Item::new(10)), Ok(()));
    /// assert_eq!(inv.remove(Item::new(10)), Ok(()));
    /// ```
    fn remove(&mut self, item: &T) -> ContainerResult<()>;

    /// Removes an item from a container at a given slot.
    fn remove_at(&mut self, slot: usize) -> ContainerResult<()>;

    /// Gets an item at a specific slot.
    fn get_at(&self, slot: usize) -> ContainerResult<T>;

    /// Swaps the slots of two items. If either `slot_a` or `slot_b` do not
    /// contain an item, then the non-empty slot is swapped with an empty one.
    fn swap(&mut self, slot_a: usize, slot_b: usize) -> ContainerResult<()>;
}
