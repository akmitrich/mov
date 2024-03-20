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

#[cfg(test)]
mod tests {
    use super::{Direction, Point, Pos, Section};

    #[test]
    fn global_position() {
        let p = Point { local: 33.45 };
        let s = Section {
            global: 43.55,
            length: 99.0,
            direction: Direction::Odd,
        };
        assert_eq!(77.0, s.global_position(&p))
    }

    #[test]
    fn change_place() {
        let mut p = Point { local: 33.45 };
        let s = Section {
            global: 43.55,
            length: 99.0,
            direction: Direction::Odd,
        };
        assert_eq!(Pos::On(34.0), p.mov(0.55, &s));
        assert_eq!(Pos::Off(-1.0), p.mov(-35.0, &s));
        assert_eq!(Pos::Off(1.0), p.mov(66.0, &s));
    }

    #[test]
    fn direction_mul() {
        let d1 = Direction::Even;
        let d2 = Direction::Odd;
        let z = Direction::No;
        assert_eq!(0.0, z * 1000.);
        assert_eq!(-7.0, d1 * 7.0);
        assert_eq!(42.5, d2 * 42.5);
        assert_eq!(13.3, d1 * (d1 * 13.3)); // without parenthesis it tries to Direction * Direction which is sad
        assert_eq!(0.0, d2 * (z * 123.)); // without parenthesis it tries to Direction * Direction which is sad
    }

    #[test]
    fn point_across_sections() {
        let mut p = Point { local: 5.0 };
        let s1 = Section {
            global: 0.0,
            length: 15.0,
            direction: Direction::Odd,
        };
        let s2 = Section {
            global: 0.0,
            length: 25.0,
            direction: Direction::Even,
        };
        if let Pos::Off(rem) = p.mov(20.0, &s1) {
            p.local = s2.accept_point(rem).unwrap();
        } else {
            panic!("Section s1 is longer than you think.");
        }
        assert_eq!(10.0, p.local);

        if let Pos::Off(rem) = p.mov(-20.0, &s1) {
            p.local = s2.accept_point(rem).unwrap();
        } else {
            panic!("Local position is greter than 20.");
        }
        assert_eq!(15.0, p.local);
    }
}
