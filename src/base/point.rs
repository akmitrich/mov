use crate::Num;

use super::Section;

pub struct Point {
    pub local: Num,
}

impl Point {
    pub fn global_position(&self, section: &Section) -> Num {
        section.global + self.local
    }

    pub fn mov(&mut self, displacement: Num, section: &Section) -> Option<Num> {
        let pos = self.local + displacement;
        if (0.0..=section.length).contains(&pos) {
            self.local = pos;
            Some(pos)
        } else {
            None
        }
    }
}
