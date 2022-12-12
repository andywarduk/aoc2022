use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::{stdout, Write};

use aoc::input::parse_input_vec;

use crate::vis::{part1vis, part2vis};

mod vis;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let map = Map::new(parse_input_vec(12, input_transform)?);

    // Run parts
    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));

    // Visualisations
    print!("Generating visualisations...");
    stdout().flush()?;

    part1vis(&map)?;
    print!(" part 1");
    stdout().flush()?;

    part2vis(&map)?;
    println!(" part2");

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
pub struct Map {
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
        self.shortest_path_internal(
            start,
            end_chk,
            neigh_chk,
            |n, w: &WorkItem<usize>| WorkItem::new(n, w.data + 1),
            |_, _, _| Ok(()),
        )
        .expect("Unexpected error")
    }

    /// Calculate the shortest path from a position to a position matching a criteria with state callback
    fn shortest_path_internal<F, G, H, I, T>(
        &self,
        start: &Pos,
        end_chk: F,
        neigh_chk: G,
        workitem: H,
        mut state_cb: I,
    ) -> Result<T, Box<dyn Error>>
    where
        F: Fn(&Pos) -> bool,
        G: Fn(u8, u8) -> bool,
        H: Fn(Pos, &WorkItem<T>) -> WorkItem<T>,
        I: FnMut(&WorkItem<T>, &VecDeque<WorkItem<T>>, &HashSet<Pos>) -> Result<(), Box<dyn Error>>,
        T: Default,
    {
        // Visited positions hash set
        let mut visited = HashSet::new();
        visited.insert(start.clone());

        // Work stack
        let mut work_queue = VecDeque::new();
        work_queue.push_back(WorkItem::new(start.clone(), T::default()));

        Ok('found: loop {
            // Get next work item
            let cur_pos = work_queue.pop_front().expect("No positions in the stack");

            // Call state callback
            state_cb(&cur_pos, &work_queue, &visited)?;

            // Finished?
            if end_chk(&cur_pos.pos) {
                break 'found cur_pos.data;
            }

            // Get neigbouring positions which match criteria
            for n in self.neighbours(&cur_pos.pos, &neigh_chk, &visited) {
                // Insert in to the visited hash map
                visited.insert(n.clone());

                // Create new work queue entry
                work_queue.push_back(workitem(n, &cur_pos));
            }
        })
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
struct WorkItem<T> {
    pos: Pos,
    data: T,
}

impl<T> WorkItem<T> {
    fn new(pos: Pos, data: T) -> Self {
        Self { pos, data }
    }
}

/// Input parsing (no-op)
fn input_transform(line: String) -> String {
    line
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

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
