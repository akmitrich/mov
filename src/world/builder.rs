use std::collections::HashMap;

use crate::{rolling::Car, track::Track, train::Train};

use super::{Item, World};

#[derive(Debug, Default)]
pub(super) struct B {
    tracks: Option<HashMap<uuid::Uuid, Track>>,
    cars: Option<HashMap<uuid::Uuid, Car>>,
    trains: Option<HashMap<uuid::Uuid, Train>>,
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

    pub fn trains(mut self, trains: Option<HashMap<uuid::Uuid, Train>>) -> Self {
        self.trains = trains;
        self
    }

    pub fn build(self) -> Option<World> {
        let (tracks, cars, trains) = (self.tracks?, self.cars?, self.trains?);
        let mut w = World::default();
        add_items_to_world(&mut w, tracks);
        add_items_to_world(&mut w, cars);
        add_items_to_world(&mut w, trains);
        Some(w)
    }
}

fn add_items_to_world<T: Item + 'static>(w: &mut World, items: HashMap<uuid::Uuid, T>) {
    for (i, t) in items {
        w.append_with_id(i, t);
    }
}
