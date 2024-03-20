use crate::{
    base::{Direction, Section},
    Num,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Track {
    section: Section,
    cars: std::collections::VecDeque<uuid::Uuid>,
}

impl Track {
    pub fn new(global: Num, length: Num, direction: Direction) -> Self {
        Self {
            section: Section {
                global,
                length,
                direction,
            },
            cars: Default::default(),
        }
    }

    pub(crate) fn accept_car(&mut self, car_id: uuid::Uuid, direction: Direction) {
        match direction {
            Direction::No => unreachable!(),
            Direction::Odd => self.cars.push_front(car_id),
            Direction::Even => self.cars.push_back(car_id),
        }
    }
}
