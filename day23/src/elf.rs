use std::collections::HashMap;

use crate::dir::Dir;

#[derive(Debug, Clone)]
pub struct Elf {
    pub x: isize,
    pub y: isize,
    pub last_move_round: Option<usize>,
}

impl Elf {
    pub fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y,
            last_move_round: None,
        }
    }

    pub fn move_to(&mut self, x: isize, y: isize) {
        self.x = x;
        self.y = y;
    }

    pub fn set_last_move_round(&mut self, round: usize) {
        self.last_move_round = Some(round);
    }

    pub fn adjacent(&self, pos_map: &HashMap<(isize, isize), usize>) -> Vec<(Dir, usize)> {
        let mut adjacent = Vec::new();

        let mut test = |x, y, dir| {
            if let Some(elem) = pos_map.get(&(x, y)) {
                adjacent.push((dir, *elem));
            }
        };

        test(self.x - 1, self.y - 1, Dir::NW);
        test(self.x, self.y - 1, Dir::N);
        test(self.x + 1, self.y - 1, Dir::NE);
        test(self.x - 1, self.y, Dir::W);
        test(self.x + 1, self.y, Dir::E);
        test(self.x - 1, self.y + 1, Dir::SW);
        test(self.x, self.y + 1, Dir::S);
        test(self.x + 1, self.y + 1, Dir::SE);

        adjacent
    }
}
