pub mod base;

pub type Num = f32;

pub fn run() {
    println!("Hello, MOV!");
}

#[cfg(test)]
mod tests {
    use crate::base::{Point, Section};

    #[test]
    fn global_position() {
        let p = Point { local: 33.45 };
        let s = Section {
            global: 43.55,
            length: 0.0,
        };
        assert_eq!(77.0, p.global_position(&s))
    }

    #[test]
    fn change_place() {
        let mut p = Point { local: 33.45 };
        let s = Section {
            global: 43.55,
            length: 99.0,
        };
        assert_eq!(Ok(34.0), p.mov(0.55, &s));
        assert_eq!(Err(-1.0), p.mov(-35.0, &s));
        assert_eq!(Err(1.0), p.mov(66.0, &s));
    }
}
