use std::borrow::Cow;
use std::cmp::{max, min};
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

fn part1vis(map: &Map) -> Result<(), Box<dyn Error>> {
    // Shortest path from START to END, allowed to go up by 1 only
    map.shortest_path_vis(
        "vis/day12-1-anim.gif",
        "vis/day12-1-final.gif",
        &map.start,
        |n| *n == map.end,
        |from, to| to <= from + 1,
    )
}

fn part2vis(map: &Map) -> Result<(), Box<dyn Error>> {
    // Shortest path from END to height 0, allowed to go down by 1 only
    map.shortest_path_vis(
        "vis/day12-2-anim.gif",
        "vis/day12-2-final.gif",
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
        self.shortest_path_internal(start, end_chk, neigh_chk, |_, _, _| Ok(()))
            .expect("Unexpected error")
            .len()
    }

    /// Calculate the shortest path from a position to a position matching a criteria with state callback
    fn shortest_path_internal<F, G, H>(
        &self,
        start: &Pos,
        end_chk: F,
        neigh_chk: G,
        mut state_cb: H,
    ) -> Result<Vec<Pos>, Box<dyn Error>>
    where
        F: Fn(&Pos) -> bool,
        G: Fn(u8, u8) -> bool,
        H: FnMut(&WorkItem, &VecDeque<WorkItem>, &HashSet<Pos>) -> Result<(), Box<dyn Error>>,
    {
        // Visited positions hash set
        let mut visited = HashSet::new();
        visited.insert(start.clone());

        // Work stack
        let mut work_queue = VecDeque::new();
        work_queue.push_back(WorkItem::new(start.clone(), Vec::new()));

        Ok('found: loop {
            // Get next work item
            let cur_pos = work_queue.pop_front().expect("No positions in the stack");

            // Call state callback
            state_cb(&cur_pos, &work_queue, &visited)?;

            // Finished?
            if end_chk(&cur_pos.pos) {
                break 'found cur_pos.path;
            }

            // Get neigbouring positions which match criteria
            for n in self.neighbours(&cur_pos.pos, &neigh_chk, &visited) {
                // Insert in to the visited hash map
                visited.insert(n.clone());
                let mut path = cur_pos.path.clone();
                path.push(n.clone());
                work_queue.push_back(WorkItem::new(n, path));
            }
        })
    }

    fn shortest_path_vis<F, G>(
        &self,
        path_anim: &str,
        path_path: &str,
        start: &Pos,
        end_chk: F,
        neigh_chk: G,
    ) -> Result<(), Box<dyn Error>>
    where
        F: Fn(&Pos) -> bool,
        G: Fn(u8, u8) -> bool,
    {
        // Calculate gif dimensions
        let gif_width = (self.max_x + 1) * GIF_SCALE;
        let gif_height = (self.max_y + 1) * GIF_SCALE;

        // Build palette
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

        // Create the animated output file
        let mut image = File::create(path_anim)?;

        // Create the gif encoder
        let mut encoder = Encoder::new(&mut image, gif_width, gif_height, &colour_map)?;

        encoder.set_repeat(Repeat::Infinite)?;

        // Create the map frame
        let map_frame = self
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

        // Draw map frame
        let base_frame = Frame {
            width: gif_width,
            height: gif_height,
            buffer: Cow::Borrowed(&*map_frame),
            delay: 2,
            ..Default::default()
        };

        encoder.write_frame(&base_frame)?;

        // Save last frame
        let mut last_frame = map_frame.clone();

        let colour_pixel = |frame: &mut Vec<u8>, x, y, col: u8| {
            let mut start = (y as usize * GIF_SCALE as usize * gif_width as usize)
                + (x as usize * GIF_SCALE as usize);

            for _ in 0..GIF_SCALE {
                let mut elem = start;

                for _ in 0..GIF_SCALE {
                    frame[elem] = (frame[elem] % 26) + (col * 26);
                    elem += 1;
                }

                start += gif_width as usize;
            }
        };

        // Callback to receive state
        let state_cb = |cur_pos: &WorkItem,
                        work_queue: &VecDeque<WorkItem>,
                        visited: &HashSet<Pos>|
         -> Result<(), Box<dyn Error>> {
            let mut next_frame = last_frame.clone();

            // Colour visited blue
            for v in visited {
                colour_pixel(&mut next_frame, v.x, v.y, 2);
            }

            // Colour work queue red
            for v in work_queue {
                colour_pixel(&mut next_frame, v.pos.x, v.pos.y, 1);
            }

            // Colour current yellow
            colour_pixel(&mut next_frame, cur_pos.pos.x, cur_pos.pos.y, 3);

            // Work out difference between this frame and the last
            let mut min_x = usize::MAX;
            let mut max_x = 0;
            let mut min_y = usize::MAX;
            let mut max_y = 0;

            for (i, (v1, v2)) in last_frame.iter().zip(next_frame.iter()).enumerate() {
                if v1 != v2 {
                    let x = i % gif_width as usize;
                    let y = i / gif_width as usize;

                    min_x = min(min_x, x);
                    max_x = max(max_x, x);
                    min_y = min(min_y, y);
                    max_y = max(max_y, y);
                }
            }

            let frame_data = next_frame
                .chunks(gif_width as usize)
                .enumerate()
                .filter_map(|(y, l)| {
                    if y >= min_y && y <= max_y {
                        Some(l[min_x..=max_x].to_vec())
                    } else {
                        None
                    }
                })
                .flatten()
                .collect::<Vec<_>>();

            let frame = Frame {
                top: min_y as u16,
                left: min_x as u16,
                width: (max_x - min_x) as u16 + 1,
                height: (max_y - min_y) as u16 + 1,
                buffer: Cow::Borrowed(&*frame_data),
                delay: 2,
                ..Default::default()
            };

            encoder.write_frame(&frame)?;

            last_frame = next_frame;

            Ok(())
        };

        let path = self.shortest_path_internal(start, end_chk, neigh_chk, state_cb)?;

        // Draw map frame with final path
        let mut next_frame = map_frame.clone();

        for p in path {
            colour_pixel(&mut next_frame, p.x, p.y, 3)
        }

        let frame = Frame {
            width: gif_width,
            height: gif_height,
            delay: 1000, // 10 seconds
            buffer: Cow::Borrowed(&*next_frame),
            ..Frame::default()
        };

        encoder.write_frame(&frame)?;

        // Draw standalone final image
        let mut image = File::create(path_path)?;

        // Create the gif encoder
        let mut encoder = Encoder::new(&mut image, gif_width, gif_height, &colour_map)?;

        encoder.set_repeat(Repeat::Infinite)?;

        let frame = Frame {
            width: gif_width,
            height: gif_height,
            buffer: Cow::Borrowed(&*next_frame),
            ..Frame::default()
        };

        encoder.write_frame(&frame)?;

        Ok(())
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
    path: Vec<Pos>,
}

impl WorkItem {
    fn new(pos: Pos, path: Vec<Pos>) -> Self {
        Self { pos, path }
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
