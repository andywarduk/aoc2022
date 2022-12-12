use std::borrow::Cow;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::{stdout, Write};

use aoc::parse_input_vec;
use gif::{Encoder, Frame, Repeat};

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let map = Map::new(parse_input_vec(12, input_transform)?);

    // Run parts
    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));

    // Visualisations
    print!("Generating visualisations...");
    stdout().flush()?;

    part1vis(&map);
    print!(" part 1");
    stdout().flush()?;

    part2vis(&map);
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

fn part1vis(map: &Map) {
    // Shortest path from START to END, allowed to go up by 1 only
    map.shortest_path_vis(
        "vis/day12-1.gif",
        &map.start,
        |n| *n == map.end,
        |from, to| to <= from + 1,
    )
}

fn part2vis(map: &Map) {
    // Shortest path from END to height 0, allowed to go down by 1 only
    map.shortest_path_vis(
        "vis/day12-2.gif",
        &map.end,
        |n| map.height(n) == 0,
        |from, to| to >= from - 1,
    )
}

const GIF_SCALE: u16 = 6;
const MIN_COLOUR_COMPONENT: u8 = 32;

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
        self.shortest_path_internal(start, end_chk, neigh_chk, |_, _, _| {})
    }

    /// Calculate the shortest path from a position to a position matching a criteria with state callback
    fn shortest_path_internal<F, G, H>(
        &self,
        start: &Pos,
        end_chk: F,
        neigh_chk: G,
        mut state_cb: H,
    ) -> usize
    where
        F: Fn(&Pos) -> bool,
        G: Fn(u8, u8) -> bool,
        H: FnMut(&WorkItem, &VecDeque<WorkItem>, &HashSet<Pos>),
    {
        // Visited positions hash set
        let mut visited = HashSet::new();
        visited.insert(start.clone());

        // Work stack
        let mut work_queue = VecDeque::new();
        work_queue.push_back(WorkItem::new(start.clone(), 0));

        'found: loop {
            // Get next work item
            let cur_pos = work_queue.pop_front().expect("No positions in the stack");

            // Call state callback
            state_cb(&cur_pos, &work_queue, &visited);

            // Finished?
            if end_chk(&cur_pos.pos) {
                break 'found cur_pos.dist;
            }

            // Get neigbouring positions which match criteria
            for n in self.neighbours(&cur_pos.pos, &neigh_chk, &visited) {
                // Insert in to the visited hash map
                visited.insert(n.clone());
                work_queue.push_back(WorkItem::new(n, cur_pos.dist + 1));
            }
        }
    }

    fn shortest_path_vis<F, G>(&self, path: &str, start: &Pos, end_chk: F, neigh_chk: G)
    where
        F: Fn(&Pos) -> bool,
        G: Fn(u8, u8) -> bool,
    {
        let gif_width = (self.max_x + 1) * GIF_SCALE;
        let gif_height = (self.max_y + 1) * GIF_SCALE;

        let colour_map: Vec<u8> = (0..=3)
            .flat_map(|i| {
                (0..26)
                    .flat_map(|j| {
                        let val = MIN_COLOUR_COMPONENT + (((255 - MIN_COLOUR_COMPONENT) / 26) * j);

                        match i {
                            0 => [0, val, 0],   // Green
                            1 => [val, 0, 0],   // Red
                            2 => [0, 0, val],   // Blue
                            3 => [val, val, 0], // Yellow
                            _ => unreachable!(),
                        }
                    })
                    .collect::<Vec<u8>>()
            })
            .collect();

        let mut image = File::create(path).unwrap();
        let mut encoder = Encoder::new(&mut image, gif_width, gif_height, &colour_map).unwrap();

        encoder.set_repeat(Repeat::Infinite).unwrap();

        let base_frame_data = self
            .heights
            .iter()
            .flat_map(|row| {
                vec![
                    row.iter()
                        .flat_map(|p| [*p; GIF_SCALE as usize])
                        .collect::<Vec<_>>();
                    GIF_SCALE as usize
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let base_frame = Frame {
            width: gif_width,
            height: gif_height,
            buffer: Cow::Borrowed(&*base_frame_data),
            delay: 2,
            ..Default::default()
        };

        encoder.write_frame(&base_frame).unwrap();

        let state_cb =
            |cur_pos: &WorkItem, work_queue: &VecDeque<WorkItem>, visited: &HashSet<Pos>| {
                let mut frame = base_frame.clone();
                let mut frame_data = base_frame_data.clone();

                let mut colour_pixel = |x, y, col: u8| {
                    let mut start = (y as usize * GIF_SCALE as usize * gif_width as usize)
                        + (x as usize * GIF_SCALE as usize);

                    for _ in 0..GIF_SCALE {
                        let mut elem = start;

                        for _ in 0..GIF_SCALE {
                            frame_data[elem] = (frame_data[elem] % 26) + (col * 26);
                            elem += 1;
                        }

                        start += gif_width as usize;
                    }
                };

                // Colour visited blue
                for v in visited {
                    colour_pixel(v.x, v.y, 2);
                }

                // Colour work queue red
                for v in work_queue {
                    colour_pixel(v.pos.x, v.pos.y, 1);
                }

                // Colour current yellow
                colour_pixel(cur_pos.pos.x, cur_pos.pos.y, 3);

                frame.buffer = Cow::Borrowed(&frame_data);

                encoder.write_frame(&frame).unwrap();
            };

        self.shortest_path_internal(start, end_chk, neigh_chk, state_cb);

        // Write dummy delay frame
        let frame = Frame {
            width: 0,
            height: 0,
            delay: 1000, // 10 seconds
            ..Frame::default()
        };

        encoder.write_frame(&frame).unwrap();
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
