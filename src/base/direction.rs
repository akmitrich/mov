use std::ops::{Mul, Neg};

use crate::Num;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
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

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Direction::No => self,
            Direction::Odd => Direction::Even,
            Direction::Even => Direction::Odd,
        }
    }
}

impl From<Num> for Direction {
    fn from(value: Num) -> Self {
        if value.abs() < 1e-6 {
            Self::No
        } else if value < 0.0 {
            Self::Even
        } else {
            Self::Odd
        }
    }
}
