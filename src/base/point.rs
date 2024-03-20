use crate::Num;

use super::{Pos, Section};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Point {
    pub local: Num,
}

impl Point {
    pub fn mov(&mut self, displacement: Num, section: &Section) -> Pos {
        let pos = self.local + displacement;
        section.shave_pos(pos).inspect(|pos| {
            self.local = pos;
        })
    }
}
