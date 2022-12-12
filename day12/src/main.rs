use std::collections::{HashSet, VecDeque};
use std::error::Error;

use aoc::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let map = Map::new(parse_input_vec(12, input_transform)?);

    // Run parts
    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));

    Ok(())
}

fn part1(map: &Map) -> usize {
    // Shortest path from START to END, allowed to go up by 1 only
    map.shortest_path(&map.start, |n| *n == map.end, |from, to| to <= from + 1)
}

fn part2(map: &Map) -> usize {
    // Shortest path from END to height 0, allowed to go down by 1 only
    map.shortest_path(&map.end, |n| map.height(n) == 0, |from, to| to >= from - 1)
}

/// Map
struct Map {
    heights: Vec<Vec<u8>>,
    start: Pos,
    end: Pos,
    max_x: u16,
    max_y: u16,
}

impl Map {
    fn new(input: Vec<String>) -> Self {
        let mut start = None;
        let mut end = None;

        // Build height vectors
        let heights = input
            .into_iter()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = Some(Pos::new(x as u16, y as u16));
                            0
                        }
                        'E' => {
                            end = Some(Pos::new(x as u16, y as u16));
                            25
                        }
                        'a'..='z' => c as u8 - b'a',
                        _ => panic!("Unexpected char"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let max_x = heights[0].len() as u16 - 1;
        let max_y = heights.len() as u16 - 1;

        Self {
            heights,
            start: start.expect("No start position found"),
            end: end.expect("No start position found"),
            max_x,
            max_y,
        }
    }

    /// Returns the height at a given position
    fn height(&self, pos: &Pos) -> u8 {
        self.heights[pos.y as usize][pos.x as usize]
    }

    /// Returns a vector of neighbours of a position
    fn neighbours<F>(&self, from_pos: &Pos, chk: F, visited: &HashSet<Pos>) -> Vec<Pos>
    where
        F: Fn(u8, u8) -> bool,
    {
        let mut neigh = Vec::with_capacity(4);
        let from_height = self.height(from_pos);

        let mut add = |x, y| {
            let to_pos = Pos::new(x, y);
            let to_height = self.height(&to_pos);

            if chk(from_height, to_height) && !visited.contains(&to_pos) {
                neigh.push(to_pos);
            }
        };

        // Left
        if from_pos.x > 0 {
            add(from_pos.x - 1, from_pos.y);
        }

        // Right
        if from_pos.x < self.max_x {
            add(from_pos.x + 1, from_pos.y);
        }

        // Up
        if from_pos.y > 0 {
            add(from_pos.x, from_pos.y - 1);
        }

        // Down
        if from_pos.y < self.max_y {
            add(from_pos.x, from_pos.y + 1);
        }

        neigh
    }

    /// Calculate the shortest path from a position to a position matching a criteria
    fn shortest_path<F, G>(&self, start: &Pos, end_chk: F, neigh_chk: G) -> usize
    where
        F: Fn(&Pos) -> bool,
        G: Fn(u8, u8) -> bool,
    {
        // Visited positions hash set
        let mut visited = HashSet::new();
        visited.insert(start.clone());

        // Work stack
        let mut stack = VecDeque::new();
        stack.push_back(WorkItem::new(start.clone(), 0));

        'found: loop {
            // Get next work item
            let cur_pos = stack.pop_front().expect("No positions in the stack");

            // Get neigbouring positions which match criteria
            let neighbours = self.neighbours(&cur_pos.pos, &neigh_chk, &visited);

            for n in neighbours {
                // Finished?
                if end_chk(&n) {
                    break 'found cur_pos.dist + 1;
                }

                // Insert in to the visited hash map
                visited.insert(n.clone());
                stack.push_back(WorkItem::new(n, cur_pos.dist + 1));
            }
        }
    }
}

/// Board position
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Pos {
    x: u16,
    y: u16,
}

impl Pos {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

/// Positions with distance for the work queue
struct WorkItem {
    pos: Pos,
    dist: usize,
}

impl WorkItem {
    fn new(pos: Pos, dist: usize) -> Self {
        Self { pos, dist }
    }
}

/// Input parsing (no-op)
fn input_transform(line: String) -> String {
    line
}

#[cfg(test)]
mod tests {
    use aoc::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test1() {
        let input = Map::new(parse_test_vec(EXAMPLE1, input_transform).unwrap());
        assert_eq!(part1(&input), 31);
        assert_eq!(part2(&input), 29);
    }
}
