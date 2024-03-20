pub mod base;

pub type Num = f32;

pub fn run() {
    println!("Hello, MOV!");
}

#[cfg(test)]
mod tests {
    use crate::base::{Direction, Point, Section};

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
        assert_eq!(Ok(34.0), p.mov(0.55, &s));
        assert_eq!(Err(-1.0), p.mov(-35.0, &s));
        assert_eq!(Err(1.0), p.mov(66.0, &s));
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
        if let Err(rem) = p.mov(20.0, &s1) {
            p.local = s2.accept_point(rem).unwrap();
        } else {
            panic!("Section s1 is longer than you think.");
        }
        assert_eq!(10.0, p.local);

        if let Err(rem) = p.mov(-20.0, &s1) {p.local=s2.accept_point(rem).unwrap();} else {
            panic!("Local position is greter than 20.");
        }
        assert_eq!(15.0, p.local);
    }
}
