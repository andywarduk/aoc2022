use std::cmp::{max, min};

const SAND_X: usize = 500;
const BORDER: usize = 2;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Rock,
    Sand,
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub x_offset: usize,
    pub content: Vec<Vec<Tile>>,
}

pub enum DropResult {
    Full,
    Out(Vec<(usize, usize)>),
    Rest(Vec<(usize, usize)>),
}

impl Map {
    /// Create new map, optionally with a floor
    pub fn new(input: &[InputEnt], floor: bool) -> Self {
        // Work out min x, max x and max y (min y is 0)
        let (mut min_x, mut max_x, mut max_y) =
            input
                .iter()
                .fold((usize::MAX, 0, 0), |(min_x, max_x, max_y), l| {
                    (
                        min(min_x, *l.iter().map(|(x, _)| x).min().unwrap()),
                        max(max_x, *l.iter().map(|(x, _)| x).max().unwrap()),
                        max(max_y, *l.iter().map(|(_, y)| y).max().unwrap()),
                    )
                });

        // Adjust for when adding the floor
        if floor {
            max_y += 2;
            min_x = SAND_X - max_y;
            max_x = SAND_X + max_y;
        };

        // Work out dimensions
        let height = max_y + BORDER;
        let width = (max_x - min_x) + (BORDER * 2);

        // Create map content vector of vectors
        let content = vec![vec![Tile::Empty; width]; height];

        // Create the struct
        let mut result = Self {
            width,
            height,
            x_offset: min_x - BORDER,
            content,
        };

        // Draw the rock lines
        for l in input {
            let (mut cur_x, mut cur_y) = l[0];

            for (next_x, next_y) in l.iter().skip(1) {
                result.line(cur_x, cur_y, *next_x, *next_y);
                (cur_x, cur_y) = (*next_x, *next_y);
            }
        }

        // Add the floor
        if floor {
            result.line(min_x, max_y, max_x, max_y)
        }

        result
    }

    /// Drops a particle of sand in to the map
    pub fn drop_sand(&mut self) -> DropResult {
        let mut x = SAND_X;
        let mut y = 0;
        let mut path = Vec::with_capacity(self.height);

        if self.get_tile(x, y) == Tile::Sand {
            return DropResult::Full;
        }

        while y < self.height - 1 {
            path.push((x, y));

            if self.tile_is_empty(x, y + 1) {
                // Go down
                y += 1;
            } else if self.tile_is_empty(x - 1, y + 1) {
                // Go down and left
                y += 1;
                x -= 1;
            } else if self.tile_is_empty(x + 1, y + 1) {
                // Go down and right
                y += 1;
                x += 1;
            } else {
                // Come to rest
                self.set_tile(x, y, Tile::Sand);
                return DropResult::Rest(path);
            }
        }

        // Fallen off of the map
        path.push((x, y));
        DropResult::Out(path)
    }

    /// Get tile at coordinate
    fn get_tile(&self, x: usize, y: usize) -> Tile {
        self.content[y][x - self.x_offset]
    }

    /// Return true if the tile at coordinate is the background
    pub fn tile_is_empty(&self, x: usize, y: usize) -> bool {
        matches!(self.get_tile(x, y), Tile::Empty)
    }

    /// Draw a horizontal or vertical line
    fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let min_x = min(x1, x2);
        let max_x = max(x1, x2);
        let min_y = min(y1, y2);
        let max_y = max(y1, y2);

        if min_x == max_x {
            for y in (min_y)..=(max_y) {
                self.set_tile(min_x, y, Tile::Rock);
            }
        } else if min_y == max_y {
            for x in (min_x)..=(max_x) {
                self.set_tile(x, min_y, Tile::Rock);
            }
        } else {
            panic!("Not horizontal or vertical")
        }
    }

    /// Sets the tile at a coordinate
    fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.content[y][x - self.x_offset] = tile;
    }
}

// Input parsing

pub type InputEnt = Vec<(usize, usize)>;

pub fn input_transform(line: String) -> InputEnt {
    line.split("->")
        .map(|seg| {
            let coord: Vec<_> = seg
                .trim()
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect();

            (coord[0], coord[1])
        })
        .collect()
}
