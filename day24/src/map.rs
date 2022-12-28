use std::{cmp::Ordering, collections::HashMap};

use crate::{
    dir::Dir,
    input::{InTile, InputEnt},
    pos::Pos,
};

#[derive(Debug, Clone)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub blizzards: Vec<HashMap<Pos, usize>>,
    pub entry: Pos,
    pub exit: Pos,
}

impl Map {
    pub fn new(input: &[InputEnt]) -> Self {
        // Get input blizzards
        let mut blizzard_pos = input
            .iter()
            .enumerate()
            .fold(Vec::new(), |blizzards, (y, row)| {
                row.iter()
                    .enumerate()
                    .fold(blizzards, |mut blizzards, (x, col)| {
                        if let InTile::Blizzard(dir) = col {
                            blizzards.push(InBlizzard::new(x, y, *dir));
                        }

                        blizzards
                    })
            });

        // Calculate inner width
        let width = input[0].len() - 2;

        // Calculate inner height
        let height = input.len() - 2;

        // Get entry point
        let entry = Pos {
            x: input[0].iter().position(|t| *t == InTile::Empty).unwrap(),
            y: 0,
        };

        // Get exit point
        let max_y = input.len() - 1;

        let exit = Pos {
            x: input[max_y]
                .iter()
                .position(|t| *t == InTile::Empty)
                .unwrap(),
            y: max_y,
        };

        // Calculate blizzard repeat
        let mut rep_h = height;
        let mut rep_w = width;

        let repeat = loop {
            match rep_h.cmp(&rep_w) {
                Ordering::Equal => break rep_h,
                Ordering::Less => rep_h += height,
                Ordering::Greater => rep_w += width,
            }
        };

        let mut blizzards = Vec::with_capacity(repeat);

        for _ in 0..repeat {
            // Add blizzard map to the set vector
            blizzards.push(blizzard_pos.iter().fold(HashMap::new(), |mut map, b| {
                *map.entry(b.pos.clone()).or_insert(0) += 1;
                map
            }));

            // Move blizzards
            (0..blizzard_pos.len()).for_each(|i| {
                let (addx, addy) = match blizzard_pos[i].dir {
                    Dir::Down => (0, 1),
                    Dir::Up => (0, -1),
                    Dir::Right => (1, 0),
                    Dir::Left => (-1, 0),
                };

                let mut new_pos = blizzard_pos[i].pos.add(addx, addy);

                // Check wrap
                if new_pos.x == 0 || new_pos.x == width + 1 {
                    new_pos.x = match addx {
                        -1 => width,
                        1 => 1,
                        _ => unreachable!(),
                    };
                }

                if new_pos.y == 0 || new_pos.y == height + 1 {
                    new_pos.y = match addy {
                        -1 => height,
                        1 => 1,
                        _ => unreachable!(),
                    };
                }

                blizzard_pos[i].pos = new_pos;
            });
        }

        Self {
            width,
            height,
            blizzards,
            entry,
            exit,
        }
    }
}

#[derive(Debug, Clone)]
struct InBlizzard {
    pos: Pos,
    dir: Dir,
}

impl InBlizzard {
    fn new(x: usize, y: usize, dir: Dir) -> Self {
        Self {
            pos: Pos { x, y },
            dir,
        }
    }
}
