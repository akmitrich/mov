use crate::{base::Point, track::Track, world::World, Num};

use super::specifications::Specifications;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Car {
    location: Point,
    specifications: Specifications,
    track: Option<uuid::Uuid>,
}

impl Car {
    pub fn new(local: Num) -> Self {
        Self {
            location: Point { local },
            specifications: Specifications::new(27.5, 100.),
            track: None,
        }
    }

    pub fn place_at(&mut self, local: Num) {
        self.location.local = local;
    }

    pub(crate) fn you_are_on(&mut self, track: uuid::Uuid) -> Option<uuid::Uuid> {
        self.track.replace(track)
    }

    pub fn global_position(&self, w: &World) -> Num {
        w.item::<Track>(self.track.unwrap())
            .map(|t| t.global_position(&self.location))
            .unwrap_or(0.0)
    }

    pub fn length(&self) -> Num {
        self.specifications.length()
    }
}

impl PartialEq for Car {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}

impl PartialOrd for Car {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.location.partial_cmp(&other.location)
    }
}
