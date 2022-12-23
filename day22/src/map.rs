use std::cmp::{max, min};

use crate::Dir;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tile {
    Open,
    Wall,
    Void,
}

#[derive(Debug, Default)]
pub struct Map {
    cell_width: usize,
    pub rows: Vec<Vec<Tile>>,
    pub edges: Vec<Edge>,
}

impl Map {
    pub fn new(input: &[String], cell_width: usize, edge_map: &[usize]) -> Self {
        // Get rows
        let rows = input
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        ' ' => Tile::Void,
                        '.' => Tile::Open,
                        '#' => Tile::Wall,
                        _ => panic!("Unrecognised tile"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // Start position
        let starty = 0_isize;
        let startx = rows[starty as usize]
            .iter()
            .position(|p| *p != Tile::Void)
            .unwrap() as isize;

        let mut x = startx;
        let mut y = starty;
        let mut dir = Dir::Right;

        let mut edges = Vec::new();

        let next_pos = |x, y, dir, amt: usize| -> (isize, isize) {
            match dir {
                Dir::Right => (x + amt as isize, y),
                Dir::Down => (x, y + amt as isize),
                Dir::Left => (x - amt as isize, y),
                Dir::Up => (x, y - amt as isize),
            }
        };

        loop {
            //println!("edge {} {} {:?}", x, y, dir);
            // Add the edge
            edges.push(Edge::new(x, y, dir, cell_width));

            // Move to new position
            (x, y) = next_pos(x, y, dir, cell_width - 1);
            //           println!("mv {} {} {:?}", x, y, dir);

            if x == startx && y == starty {
                break;
            }

            // Get next directions to check
            let next_dirs = match dir {
                Dir::Right => [(1, -1, Dir::Up), (1, 0, Dir::Right), (0, 0, Dir::Down)],
                Dir::Down => [(1, 1, Dir::Right), (0, 1, Dir::Down), (0, 0, Dir::Left)],
                Dir::Left => [(-1, 1, Dir::Down), (-1, 0, Dir::Left), (0, 0, Dir::Up)],
                Dir::Up => [(-1, -1, Dir::Left), (0, -1, Dir::Up), (0, 0, Dir::Right)],
            };

            // Get next position and direction
            (x, y, dir) = next_dirs
                .into_iter()
                .find_map(|(xadd, yadd, d)| {
                    let chkx = x + xadd;
                    let chky = y + yadd;
                    //                    println!("chk {} {} {:?}", chkx, chky, d);
                    if Self::get_tile_internal(&rows, chkx, chky) == Tile::Void {
                        None
                    } else {
                        Some((chkx, chky, d))
                    }
                })
                .unwrap();
        }

        // TODO manual for now
        edge_map
            .iter()
            .enumerate()
            .for_each(|(i, j)| edges[i].pair = *j);

        Self {
            cell_width,
            rows,
            edges,
        }
    }

    pub fn get_tile(&self, x: isize, y: isize) -> Tile {
        Self::get_tile_internal(&self.rows, x, y)
    }

    fn get_tile_internal(rows: &Vec<Vec<Tile>>, x: isize, y: isize) -> Tile {
        if x < 0 || y < 0 || y as usize >= rows.len() || x as usize >= rows[y as usize].len() {
            Tile::Void
        } else {
            rows[y as usize][x as usize]
        }
    }

    pub fn wrap(&self, x: isize, xadd: isize, y: isize, yadd: isize) -> (isize, isize) {
        match (xadd.signum(), yadd.signum()) {
            (-1, 0) => (self.rows[y as usize].len() as isize - 1, y),
            (1, 0) => (
                self.rows[y as usize]
                    .iter()
                    .position(|p| *p != Tile::Void)
                    .unwrap() as isize,
                y,
            ),
            (0, -1) => (
                x,
                self.rows
                    .iter()
                    .enumerate()
                    .filter_map(|(i, row)| {
                        if row.len() > x as usize && row[x as usize] != Tile::Void {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .last()
                    .unwrap() as isize,
            ),
            (0, 1) => (
                x,
                self.rows
                    .iter()
                    .position(|row| row.len() > x as usize && row[x as usize] != Tile::Void)
                    .unwrap() as isize,
            ),
            _ => panic!("Unexpected wrap"),
        }
    }

    pub fn cubewrap(&self, x: isize, xadd: isize, y: isize, yadd: isize) -> (isize, isize, Dir) {
        // Find which edge we're crossing
        let mut out_edge = None;

        for e in self.edges.iter() {
            if e.on(x, y) {
                // On the edge - check direction

                match (xadd.signum(), yadd.signum()) {
                    (-1, 0) => {
                        // Left
                        if e.dir == Dir::Up {
                            out_edge = Some(e);
                            break;
                        }
                    }
                    (1, 0) => {
                        // Right
                        if e.dir == Dir::Down {
                            out_edge = Some(e);
                            break;
                        }
                    }
                    (0, -1) => {
                        // Up
                        if e.dir == Dir::Right {
                            out_edge = Some(e);
                            break;
                        }
                    }
                    (0, 1) => {
                        // Down
                        if e.dir == Dir::Left {
                            out_edge = Some(e);
                            break;
                        }
                    }
                    _ => panic!("Unexpected wrap"),
                }
            }
        }

        let out_edge = out_edge.expect("Out edge not found");
        let out_intersect = out_edge.intersect(x, y).unwrap();
        println!("out {:?} @ {}", out_edge, out_intersect);

        let in_edge = &self.edges[out_edge.pair];
        let in_intersect = self.cell_width as isize - 1 - out_intersect;
        println!("in {:?} @ {}", in_edge, in_intersect);

        match in_edge.dir {
            Dir::Up => (in_edge.x1, in_edge.y1 - in_intersect, Dir::Right),
            Dir::Down => (in_edge.x1, in_edge.y1 + in_intersect, Dir::Left),
            Dir::Left => (in_edge.x1 - in_intersect, in_edge.y1, Dir::Up),
            Dir::Right => (in_edge.x1 + in_intersect, in_edge.y1, Dir::Down),
        }
    }
}

#[derive(Debug)]
pub struct Edge {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
    dir: Dir,
    pair: usize,
}

impl Edge {
    fn new(x: isize, y: isize, dir: Dir, len: usize) -> Self {
        let (x2, y2) = match dir {
            Dir::Up => (x, y - (len as isize - 1)),
            Dir::Down => (x, y + (len as isize - 1)),
            Dir::Left => (x - (len as isize - 1), y),
            Dir::Right => (x + (len as isize - 1), y),
        };

        Self {
            x1: x,
            y1: y,
            x2,
            y2,
            dir,
            pair: 0,
        }
    }

    fn on(&self, x: isize, y: isize) -> bool {
        let min_x = min(self.x1, self.x2);
        let max_x = max(self.x1, self.x2);
        let min_y = min(self.y1, self.y2);
        let max_y = max(self.y1, self.y2);

        x >= min_x && x <= max_x && y >= min_y && y <= max_y
    }

    fn intersect(&self, x: isize, y: isize) -> Option<isize> {
        if self.on(x, y) {
            Some(max((x - self.x1).abs(), (y - self.y1).abs()))
        } else {
            None
        }
    }
}
