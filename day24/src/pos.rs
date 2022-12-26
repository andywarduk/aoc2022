use std::cmp::{max, min};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn add(&self, addx: isize, addy: isize) -> Self {
        Self {
            x: (self.x as isize + addx) as usize,
            y: (self.y as isize + addy) as usize,
        }
    }

    pub fn dist(&self, other: &Self) -> usize {
        (max(self.x, other.x) - min(self.x, other.x))
            + (max(self.y, other.y) - min(self.y, other.y))
    }
}
