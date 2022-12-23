use std::{
    cmp::{max, min},
    collections::{HashMap, VecDeque},
    error::Error,
    fmt,
    slice::Iter,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(23, input_transform)?;

    let elves = Elves::build(input);

    // Run parts
    println!("Part 1: {}", part1(elves.clone()));
    println!("Part 2: {}", part2(elves));

    Ok(())
}

fn part1(mut elves: Elves) -> usize {
    // Run 10 rounds
    for _ in 0..10 {
        elves.move_all();
    }

    // Get bounding box
    let (minx, miny, maxx, maxy) = elves.bbox();

    // Calculate area
    let area = (((maxx - minx) + 1) * ((maxy - miny) + 1)) as usize;

    // Calculate empty squares
    area - elves.len()
}

fn part2(mut elves: Elves) -> u64 {
    let mut round = 0;

    // Run until no elves move
    loop {
        round += 1;

        if elves.move_all() == 0 {
            break;
        }
    }

    round
}

#[derive(Debug, Clone)]
enum Dir {
    NW,
    N,
    NE,
    W,
    E,
    SW,
    S,
    SE,
}

#[derive(Clone)]
struct Elves {
    elves: Vec<Elf>,
    pos_map: HashMap<(isize, isize), usize>,
    move_pref: VecDeque<Dir>,
}

impl Elves {
    fn build(input: Vec<InputEnt>) -> Self {
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
        }
    }

    fn move_all(&mut self) -> usize {
        // Calculate proposed moves for each elf
        let moves = self
            .iter()
            .map(|elf| {
                let adjacent = elf.adjacent(&self.pos_map);

                if adjacent.is_empty() {
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
                }
            })
            .collect::<Vec<_>>();

        // Work out new positions
        let new_pos = moves
            .iter()
            .enumerate()
            .map(|(i, mv)| {
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

        // Move the elves
        for (i, pos) in new_pos.into_iter().enumerate() {
            if *new_pos_map.get(&pos).unwrap() == 1 {
                self.pos_map
                    .remove(&(self.elves[i].x, self.elves[i].y))
                    .unwrap();
                self.elves[i].move_to(pos.0, pos.1);
                self.pos_map.insert(pos, i);
            }
        }

        // Update move preferences
        self.next_pref();

        // Return number of moves
        moves.iter().filter(|m| m.is_some()).count()
    }

    fn next_pref(&mut self) {
        let p = self.move_pref.pop_front().unwrap();
        self.move_pref.push_back(p);
    }

    fn iter(&self) -> Iter<Elf> {
        self.elves.iter()
    }

    fn len(&self) -> usize {
        self.elves.len()
    }

    fn bbox(&self) -> (isize, isize, isize, isize) {
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

#[derive(Debug, Clone)]
struct Elf {
    x: isize,
    y: isize,
}

impl Elf {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn move_to(&mut self, x: isize, y: isize) {
        self.x = x;
        self.y = y;
    }

    fn adjacent(&self, pos_map: &HashMap<(isize, isize), usize>) -> Vec<(Dir, usize)> {
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

// Input parsing

type InputEnt = Vec<bool>;

fn input_transform(line: String) -> InputEnt {
    line.chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!("Unexpected character"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "##
#.
..
##
";

    const EXAMPLE2: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let mut elves = Elves::build(input);

        elves.move_all();
        println!("{:?}", elves);
        assert_eq!(format!("{:?}", elves), "##\n..\n#.\n.#\n#.\n");
        elves.move_all();
        println!("{:?}", elves);
        assert_eq!(format!("{:?}", elves), ".##.\n#...\n...#\n....\n.#..\n");
        elves.move_all();
        println!("{:?}", elves);
        assert_eq!(
            format!("{:?}", elves),
            "..#..\n....#\n#....\n...#.\n.....\n.#...\n"
        );
        elves.move_all();
        println!("{:?}", elves);
        assert_eq!(
            format!("{:?}", elves),
            "..#..\n....#\n#....\n...#.\n.....\n.#...\n"
        );
    }

    #[test]
    fn test2() {
        let input = parse_test_vec(EXAMPLE2, input_transform).unwrap();
        let elves = Elves::build(input);

        assert_eq!(part1(elves.clone()), 110);
        assert_eq!(part2(elves), 20);
    }
}
