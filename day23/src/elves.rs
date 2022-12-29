use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::slice::Iter;

use crate::{dir::Dir, elf::Elf, input::InputEnt};

#[derive(Clone)]
pub struct Elves {
    elves: Vec<Elf>,
    pos_map: HashMap<(isize, isize), usize>,
    move_pref: VecDeque<Dir>,
    rounds: usize,
}

impl Elves {
    pub fn build(input: Vec<InputEnt>) -> Self {
        let elves = input
            .iter()
            .enumerate()
            .fold(Vec::new(), |elves, (y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, elf)| **elf)
                    .fold(elves, |mut elves, (x, _)| {
                        elves.push(Elf::new(x as isize, y as isize));
                        elves
                    })
            });

        let pos_map = elves
            .iter()
            .enumerate()
            .map(|(i, e)| ((e.x, e.y), i))
            .collect();

        Self {
            elves,
            pos_map,
            move_pref: vec![Dir::N, Dir::S, Dir::W, Dir::E].into(),
            rounds: 0,
        }
    }

    pub fn move_all(&mut self) -> usize {
        let new_pos = self
            .iter()
            .enumerate()
            .map(|(i, elf)| {
                // Calculate proposed moves for each elf
                let adjacent = elf.adjacent(&self.pos_map);

                let mv = if adjacent.is_empty() {
                    None
                } else {
                    self.move_pref.iter().cloned().find(|mv| {
                        adjacent.iter().all(|(dir, _)| match mv {
                            Dir::N => !matches!(dir, Dir::NW | Dir::N | Dir::NE),
                            Dir::S => !matches!(dir, Dir::SW | Dir::S | Dir::SE),
                            Dir::E => !matches!(dir, Dir::NE | Dir::E | Dir::SE),
                            Dir::W => !matches!(dir, Dir::NW | Dir::W | Dir::SW),
                            _ => unreachable!(),
                        })
                    })
                };

                // Calculate new position for each elf
                let (x, y) = (self.elves[i].x, self.elves[i].y);

                match mv {
                    None => (x, y),
                    Some(dir) => match dir {
                        Dir::N => (x, y - 1),
                        Dir::E => (x + 1, y),
                        Dir::S => (x, y + 1),
                        Dir::W => (x - 1, y),
                        _ => unreachable!(),
                    },
                }
            })
            .collect::<Vec<_>>();

        // Count position clashes
        let new_pos_map = new_pos.iter().fold(HashMap::new(), |mut pos_map, p| {
            *(pos_map.entry(*p).or_insert(0)) += 1;
            pos_map
        });

        // Update number of rounds
        self.rounds += 1;

        // Move the elves
        let mut moves = 0;

        for (i, pos) in new_pos.into_iter().enumerate() {
            // Make sure only one elf wants this space
            if *new_pos_map.get(&pos).unwrap() == 1 {
                let (x, y) = pos;

                let ex = self.elves[i].x;
                let ey = self.elves[i].y;

                // Is the elf moving?
                if ex != x || ey != y {
                    // Remove from the position map
                    self.pos_map.remove(&(ex, ey)).unwrap();

                    // Move the elf
                    self.elves[i].move_to(x, y);

                    // Set last move round
                    self.elves[i].set_last_move_round(self.rounds);

                    // Insert new position in to the position map
                    self.pos_map.insert(pos, i);

                    // Increment moves
                    moves += 1;
                }
            }
        }

        // Update move preferences
        self.next_pref();

        // Return number of moves
        moves
    }

    fn next_pref(&mut self) {
        let p = self.move_pref.pop_front().unwrap();
        self.move_pref.push_back(p);
    }

    pub fn iter(&self) -> Iter<Elf> {
        self.elves.iter()
    }

    pub fn len(&self) -> usize {
        self.elves.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn rounds(&self) -> usize {
        self.rounds
    }

    pub fn bbox(&self) -> (isize, isize, isize, isize) {
        self.iter().fold(
            (isize::MAX, isize::MAX, isize::MIN, isize::MIN),
            |(minx, miny, maxx, maxy), e| {
                (
                    min(minx, e.x),
                    min(miny, e.y),
                    max(maxx, e.x),
                    max(maxy, e.y),
                )
            },
        )
    }
}

impl fmt::Debug for Elves {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (minx, miny, maxx, maxy) = self.bbox();

        let mut map = String::new();
        for y in miny..=maxy {
            for x in minx..=maxx {
                if self.iter().any(|e| e.x == x && e.y == y) {
                    map.push('#');
                } else {
                    map.push('.');
                }
            }
            map.push('\n');
        }

        f.write_str(&map)
    }
}
