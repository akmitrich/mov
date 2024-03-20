use super::{Direction, Point, Pos};
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

    pub fn shave_pos(&self, pos: Num) -> Pos {
        if pos < 0.0 {
            Pos::Off(pos)
        } else if pos > self.length {
            Pos::Off(pos - self.length)
        } else {
            Pos::On(pos)
        }
    }

    pub fn accept_point(&self, remainder: Num) -> Pos {
        let pos = match Direction::from(remainder) {
            Direction::Even => self.length + remainder,
            Direction::Odd => remainder,
            _ => 0.0,
        };
        self.shave_pos(pos)
    }
}
