use crate::Num;

use super::Section;

pub struct Point {
    pub local: Num,
}

impl Point {
    pub fn global_position(&self, section: &Section) -> Num {
        section.global + self.local
    }

    pub fn mov(&mut self, displacement: Num, section: &Section) -> Result<Num, Num> {
        let pos = self.local + displacement;
        if pos < 0.0 {
            Err(pos)
        } else if pos > section.length {
            Err(pos - section.length)
        } else {
            self.local = pos;
            Ok(pos)
        }
    }
}
