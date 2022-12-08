use std::error::Error;
use std::iter::{empty, repeat, zip};

use aoc::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(8, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> usize {
    CoordIterator::new(input)
        .filter(|(x, y)| visible(input, *x, *y))
        .count()
}

fn part2(input: &[InputEnt]) -> usize {
    CoordIterator::new(input)
        .map(|(x, y)| scenic_score(input, x, y))
        .max()
        .unwrap()
}

/// Tests if a tree at the given position is visible
fn visible(input: &[InputEnt], tx: usize, ty: usize) -> bool {
    visible_scan(input, tx, ty, Direction::Left)
        || visible_scan(input, tx, ty, Direction::Right)
        || visible_scan(input, tx, ty, Direction::Up)
        || visible_scan(input, tx, ty, Direction::Down)
}

/// Tests if a tree at the given position is visible from a given direction
fn visible_scan(input: &[InputEnt], tx: usize, ty: usize, direction: Direction) -> bool {
    let height = input[ty][tx];

    for (x, y) in DirectionIterator::new(input, tx, ty, direction, true) {
        if input[y][x] >= height {
            return false;
        }
    }

    true
}

/// Returns the scenic score for a tree at a given position
fn scenic_score(input: &[InputEnt], tx: usize, ty: usize) -> usize {
    // Left
    let lscore = scenic_score_scan(input, tx, ty, Direction::Left);

    // Right
    let rscore = scenic_score_scan(input, tx, ty, Direction::Right);

    // Up
    let uscore = scenic_score_scan(input, tx, ty, Direction::Up);

    // Down
    let dscore = scenic_score_scan(input, tx, ty, Direction::Down);

    lscore * rscore * uscore * dscore
}

/// Returns the distance to the last visible tree in a given direction
fn scenic_score_scan(input: &[InputEnt], tx: usize, ty: usize, direction: Direction) -> usize {
    let height = input[ty][tx];
    let mut dist = 0;

    for (x, y) in DirectionIterator::new(input, tx, ty, direction, false) {
        dist += 1;

        if input[y][x] >= height {
            break;
        }
    }

    dist
}

type CoordIterItem = (usize, usize);
type CoordIter = Box<dyn Iterator<Item = CoordIterItem>>;

/// Iterator for all positions in the input vector of vectors
struct CoordIterator {
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
    finished: bool,
}

impl CoordIterator {
    fn new(input: &[InputEnt]) -> Self {
        let max_x = input[0].len();
        let max_y = input.len();

        Self {
            x: 0,
            y: 0,
            max_x,
            max_y,
            finished: max_x == 0 || max_y == 0,
        }
    }
}

impl Iterator for CoordIterator {
    type Item = CoordIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else {
            let item = Some((self.x, self.y));

            self.x += 1;

            if self.x == self.max_x {
                self.x = 0;
                self.y += 1;

                if self.y == self.max_y {
                    self.finished = true;
                }
            }

            item
        }
    }
}

/// Direction enumeration
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Iterator for all coordinates in the input vector from a given position, optionally reverse order
struct DirectionIterator {
    iterator: CoordIter,
}

impl DirectionIterator {
    fn new(
        input: &[InputEnt],
        x: usize,
        y: usize,
        direction: Direction,
        reverse: bool,
    ) -> DirectionIterator {
        let iterator: CoordIter = match direction {
            Direction::Up => {
                Box::new(zip(repeat(x), Self::coord_iterator(0, y, !reverse))) as CoordIter
            }
            Direction::Down => Box::new(zip(
                repeat(x),
                Self::coord_iterator(y + 1, input.len(), reverse),
            )) as CoordIter,
            Direction::Left => {
                Box::new(zip(Self::coord_iterator(0, x, !reverse), repeat(y))) as CoordIter
            }
            Direction::Right => Box::new(zip(
                Self::coord_iterator(x + 1, input[0].len(), reverse),
                repeat(y),
            )) as CoordIter,
        };

        Self { iterator }
    }

    fn coord_iterator(start: usize, end: usize, reverse: bool) -> Box<dyn Iterator<Item = usize>> {
        let iter: Box<dyn Iterator<Item = usize>> = if end <= start {
            Box::new(empty::<usize>())
        } else if reverse {
            Box::new((start..end).rev())
        } else {
            Box::new(start..end)
        };

        iter
    }
}

impl Iterator for DirectionIterator {
    type Item = CoordIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

// Input parsing

type InputEnt = Vec<u8>;

fn input_transform(line: String) -> InputEnt {
    line.chars().map(|c| c as u8 - b'0').collect()
}

#[cfg(test)]
mod tests {
    use aoc::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();

        assert!(visible(&input, 1, 1));
        assert!(visible(&input, 2, 1));
        assert!(!visible(&input, 3, 1));
        assert!(visible(&input, 1, 2));
        assert!(!visible(&input, 2, 2));
        assert!(visible(&input, 3, 2));
        assert!(!visible(&input, 1, 3));
        assert!(visible(&input, 2, 3));
        assert!(!visible(&input, 3, 3));

        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn test2() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();

        assert_eq!(scenic_score(&input, 2, 1), 4);
        assert_eq!(scenic_score(&input, 2, 3), 8);

        assert_eq!(part2(&input), 8);
    }

    #[test]
    fn test3() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();

        assert_eq!(
            DirectionIterator::new(&input, 2, 2, Direction::Left, false)
                .collect::<Vec<CoordIterItem>>(),
            [(1, 2), (0, 2)]
        );
        assert_eq!(
            DirectionIterator::new(&input, 2, 2, Direction::Right, false)
                .collect::<Vec<CoordIterItem>>(),
            [(3, 2), (4, 2)]
        );
        assert_eq!(
            DirectionIterator::new(&input, 2, 2, Direction::Up, false)
                .collect::<Vec<CoordIterItem>>(),
            [(2, 1), (2, 0)]
        );
        assert_eq!(
            DirectionIterator::new(&input, 2, 2, Direction::Down, false)
                .collect::<Vec<CoordIterItem>>(),
            [(2, 3), (2, 4)]
        );

        assert_eq!(
            DirectionIterator::new(&input, 0, 0, Direction::Left, false)
                .collect::<Vec<CoordIterItem>>(),
            []
        );
        assert_eq!(
            DirectionIterator::new(&input, 0, 0, Direction::Right, false)
                .collect::<Vec<CoordIterItem>>(),
            [(1, 0), (2, 0), (3, 0), (4, 0)]
        );
        assert_eq!(
            DirectionIterator::new(&input, 0, 0, Direction::Up, false)
                .collect::<Vec<CoordIterItem>>(),
            []
        );
        assert_eq!(
            DirectionIterator::new(&input, 0, 0, Direction::Down, false)
                .collect::<Vec<CoordIterItem>>(),
            [(0, 1), (0, 2), (0, 3), (0, 4)]
        );

        assert_eq!(
            DirectionIterator::new(&input, 4, 4, Direction::Left, false)
                .collect::<Vec<CoordIterItem>>(),
            [(3, 4), (2, 4), (1, 4), (0, 4)]
        );
        assert_eq!(
            DirectionIterator::new(&input, 4, 4, Direction::Right, false)
                .collect::<Vec<CoordIterItem>>(),
            []
        );
        assert_eq!(
            DirectionIterator::new(&input, 4, 4, Direction::Up, false)
                .collect::<Vec<CoordIterItem>>(),
            [(4, 3), (4, 2), (4, 1), (4, 0)]
        );
        assert_eq!(
            DirectionIterator::new(&input, 4, 4, Direction::Down, false)
                .collect::<Vec<CoordIterItem>>(),
            []
        );
    }
}
