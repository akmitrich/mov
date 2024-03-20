use crate::{base::Point, Num};

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
            specifications: Specifications::new(27.5),
            track: None,
        }
    }

    pub(crate) fn you_are_on(&mut self, track: uuid::Uuid) -> Option<uuid::Uuid> {
        self.track.replace(track)
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
