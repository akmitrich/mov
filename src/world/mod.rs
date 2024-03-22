mod disk;
mod item;

pub use item::Item;

#[derive(Debug, Default)]
pub struct World {
    items: std::collections::HashMap<uuid::Uuid, Box<dyn Item>>,
}

impl World {
    pub fn append<T: Item + 'static>(&mut self, x: T) -> uuid::Uuid {
        let id = uuid::Uuid::new_v4();
        self.items.insert(id, Box::new(x));
        id
    }

    pub fn item<T: Item + 'static>(&self, id: uuid::Uuid) -> Option<&T> {
        self.items
            .get(&id)
            .and_then(|item| item.as_any().downcast_ref())
    }

    pub fn item_mut<T: Item + 'static>(&mut self, id: uuid::Uuid) -> Option<&mut T> {
        self.items
            .get_mut(&id)
            .and_then(|item| item.as_any_mut().downcast_mut())
    }

    pub fn items_of_type<T: Item + 'static>(&self) -> std::collections::HashMap<uuid::Uuid, &T> {
        self.items
            .iter()
            .filter_map(|(id, item)| item.as_any().downcast_ref().map(|item| (*id, item)))
            .collect()
    }

    pub fn save(&self, path: impl AsRef<std::path::Path>) {
        disk::save(path, self);
    }
}
