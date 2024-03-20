use crate::Num;

use super::Section;

pub struct Point {
    pub local: Num,
}

impl Point {
    pub fn mov(&mut self, displacement: Num, section: &Section) -> Result<Num, Num> {
        let pos = self.local + displacement;
        section.shave_pos(pos).map(|pos| {
            self.local = pos;
            pos
        })
    }
}
