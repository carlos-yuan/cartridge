use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Item<T> {
    pub object: T,
    expiry: Option<Instant>,
}

impl<T> Item<T> {
    // Creates a new cache item.
    pub fn new(object: T, item_duration: Option<Duration>) -> Self {
        let expiry = item_duration.map(|duration| Instant::now() + duration);
        Item { object, expiry }
    }

    // Returns true if the item has expired.
    pub fn expired(&self) -> bool {
        self.expiry
            .map(|expiry| expiry < Instant::now())
            .unwrap_or(false)
    }
}

