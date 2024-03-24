use std::collections::HashMap;

use crate::{rolling::Car, track::Track};

use super::{Item, World};

#[derive(Debug, Default)]
pub(super) struct B {
    tracks: Option<HashMap<uuid::Uuid, Track>>,
    cars: Option<HashMap<uuid::Uuid, Car>>,
}

impl B {
    pub fn tracks(mut self, tracks: Option<HashMap<uuid::Uuid, Track>>) -> Self {
        self.tracks = tracks;
        self
    }

    pub fn cars(mut self, cars: Option<HashMap<uuid::Uuid, Car>>) -> Self {
        self.cars = cars;
        self
    }

    pub fn build(self) -> Option<World> {
        let (tracks, cars) = (self.tracks?, self.cars?);
        let mut w = World::default();
        add_items_to_world(&mut w, tracks);
        add_items_to_world(&mut w, cars);
        Some(w)
    }
}

fn add_items_to_world<T: Item + 'static>(w: &mut World, items: HashMap<uuid::Uuid, T>) {
    for (i, t) in items {
        w.append_with_id(i, t);
    }
}
