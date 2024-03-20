use std::ops::Mul;

use crate::Num;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    No,
    Odd,
    Even,
}

impl Mul<Num> for Direction {
    type Output = Num;

    fn mul(self, rhs: Num) -> Self::Output {
        match self {
            Direction::No => 0.0,
            Direction::Odd => rhs,
            Direction::Even => -rhs,
        }
    }
}
