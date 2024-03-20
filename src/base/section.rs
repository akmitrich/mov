use super::{Direction, Point};
use crate::Num;

pub struct Section {
    pub global: Num,
    pub length: Num,
    pub direction: Direction,
}

impl Section {
    pub fn global_position(&self, point: &Point) -> Num {
        match self.direction {
            Direction::No => 0.0,
            Direction::Odd => self.global + point.local,
            Direction::Even => self.global - point.local,
        }
    }
}
