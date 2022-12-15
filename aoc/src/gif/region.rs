use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

pub struct Region {
    top: u16,
    left: u16,
    bottom: u16,
    right: u16,
}

impl Region {
    pub fn new(top: u16, left: u16, bottom: u16, right: u16) -> Self {
        Region {
            top,
            left,
            bottom,
            right,
        }
    }

    pub fn left(&self) -> u16 {
        self.left
    }

    pub fn top(&self) -> u16 {
        self.top
    }

    pub fn width(&self) -> u16 {
        (self.right - self.left) + 1
    }

    pub fn height(&self) -> u16 {
        (self.bottom - self.top) + 1
    }

    pub fn contains_y(&self, y: u16) -> bool {
        y >= self.top && y <= self.bottom
    }

    pub fn x_range(&self) -> RangeInclusive<usize> {
        (self.left as usize)..=(self.right as usize)
    }

    /// Initialises a region for the max region calculation
    pub fn max_init() -> Self {
        Self {
            top: u16::MAX,
            left: u16::MAX,
            bottom: 0,
            right: 0,
        }
    }

    /// Adds a coordinate in the max region calculation
    pub fn max_add(&mut self, x: u16, y: u16) {
        self.top = min(self.top, y);
        self.left = min(self.left, x);
        self.bottom = max(self.bottom, y);
        self.right = max(self.right, x);
    }

    /// Returns true if max has been calculated successfully
    pub fn max_valid(&self) -> bool {
        self.top != u16::MAX
    }
}
