mod direction;
mod point;
mod section;

pub use direction::*;
pub use point::*;
pub use section::*;

use crate::Num;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pos {
    On(Num),
    Off(Num),
}

impl Pos {
    pub fn unwrap(self) -> Num {
        match self {
            Pos::On(pos) => pos,
            Pos::Off(rem) => panic!("Unwrap position which is off the section by {}.", rem),
        }
    }

    pub fn inspect<F: FnOnce(Num)>(self, f: F) -> Self {
        if let Self::On(t) = self {
            f(t);
        }
        self
    }
}
