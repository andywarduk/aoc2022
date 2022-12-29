use std::{
    collections::{HashSet, VecDeque},
    error::Error,
};

use aoc::gif::Gif;

use self::palette::COLOUR_MAP;
use self::pos::Pos;
use self::workitem::WorkItem;

mod palette;
mod pos;
mod workitem;

/// Map
pub struct Map {
    heights: Vec<Vec<u8>>,
    start: Pos,
    end: Pos,
    max_x: u16,
    max_y: u16,
}

impl Map {
    pub fn new(input: Vec<String>) -> Self {
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

    /// Returns the start position
    pub fn start(&self) -> &Pos {
        &self.start
    }

    /// Returns the end position
    pub fn end(&self) -> &Pos {
        &self.end
    }

    /// Returns the height at a given position
    pub fn height(&self, pos: &Pos) -> u8 {
        self.heights[pos.y as usize][pos.x as usize]
    }

    /// Calculate the shortest path from a position to a position matching a criteria
    pub fn shortest_path<F, G>(&self, start: &Pos, end_chk: F, neigh_chk: G) -> usize
    where
        F: Fn(&Pos) -> bool,
        G: Fn(u8, u8) -> bool,
    {
        self.shortest_path_internal(
            start,
            neigh_chk,
            |n, w| {
                WorkItem::new(
                    n,
                    match w {
                        Some(w) => w.data + 1,
                        None => 0,
                    },
                )
            },
            |work_item: &WorkItem<usize>, _, _, result| -> Result<bool, Box<dyn Error>> {
                if end_chk(&work_item.pos) {
                    *result = Some(work_item.data);
                    Ok(false)
                } else {
                    Ok(true)
                }
            },
        )
        .expect("Unexpected error")
        .expect("Shortest path not found")
    }

    /// Generate animated GIF of the shortest path algorithm
    /// and static GIF of the final solution
    pub fn shortest_path_vis<F, G>(
        &self,
        path_anim: &str,
        path_path: &str,
        scale: u16,
        start: &Pos,
        end_chk: F,
        neigh_chk: G,
    ) -> Result<(), Box<dyn Error>>
    where
        F: Fn(&Pos) -> bool,
        G: Fn(u8, u8) -> bool,
    {
        // Create the animated gif
        let mut gif = Gif::new(
            path_anim,
            &COLOUR_MAP,
            self.max_x + 1,
            self.max_y + 1,
            scale,
            scale,
        )?;

        // Create the map frame
        let map_frame = self
            .heights
            .iter()
            .map(|row| row.to_vec())
            .collect::<Vec<_>>();

        // Draw map frame
        gif.draw_frame(map_frame.clone(), 2)?;

        let colour_pixel = |frame: &mut Vec<Vec<u8>>, x, y, col: u8| {
            frame[y as usize][x as usize] = (frame[y as usize][x as usize] % 26) + (col * 26);
        };

        let mut last_len = 0;

        // Callback to receive state
        let state_cb = |cur_pos: &WorkItem<Vec<Pos>>,
                        work_queue: &VecDeque<WorkItem<Vec<Pos>>>,
                        visited: &HashSet<Pos>,
                        result: &mut Option<Vec<Pos>>|
         -> Result<bool, Box<dyn Error>> {
            // Finished?
            if result.is_none() && end_chk(&cur_pos.pos) {
                // Save first result
                *result = Some(cur_pos.data.clone());
            }

            // Only plot if path length has increased
            if cur_pos.data.len() != last_len {
                last_len = cur_pos.data.len();

                // Clone the map frame
                let mut next_frame = map_frame.clone();

                // Colour visited blue
                for v in visited {
                    colour_pixel(&mut next_frame, v.x, v.y, 2);
                }

                // Colour work queue red
                for v in work_queue {
                    colour_pixel(&mut next_frame, v.pos.x, v.pos.y, 1);
                }

                // Colour current path / final path yellow
                let plot_path = match result {
                    Some(p) => p,
                    None => &cur_pos.data,
                };

                for p in plot_path.iter() {
                    colour_pixel(&mut next_frame, p.x, p.y, 3)
                }

                // Draw the frame
                gif.draw_frame(next_frame, 2)?;
            }

            Ok(true)
        };

        // Get shortest path
        let path = self
            .shortest_path_internal(
                start,
                neigh_chk,
                |n, w| {
                    let mut path = match w {
                        Some(w) => w.data.clone(),
                        None => Vec::new(),
                    };

                    path.push(n.clone());

                    WorkItem::new(n, path)
                },
                state_cb,
            )?
            .expect("No path found");

        // Draw map frame with final path
        let mut next_frame = map_frame.clone();

        for p in path {
            colour_pixel(&mut next_frame, p.x, p.y, 3)
        }

        gif.draw_frame(next_frame.clone(), 1000)?;

        // Draw standalone final image
        let mut gif = Gif::new(
            path_path,
            &COLOUR_MAP,
            self.max_x + 1,
            self.max_y + 1,
            scale,
            scale,
        )?;

        gif.draw_frame(next_frame, 2)?;

        Ok(())
    }

    /// Calculate the shortest path from a position to a position matching a criteria with state callback
    fn shortest_path_internal<G, H, I, T>(
        &self,
        start: &Pos,
        neigh_chk: G,
        workitem: H,
        mut state_cb: I,
    ) -> Result<Option<T>, Box<dyn Error>>
    where
        G: Fn(u8, u8) -> bool,
        H: Fn(Pos, Option<&WorkItem<T>>) -> WorkItem<T>,
        I: FnMut(
            &WorkItem<T>,
            &VecDeque<WorkItem<T>>,
            &HashSet<Pos>,
            &mut Option<T>,
        ) -> Result<bool, Box<dyn Error>>,
        T: Default,
    {
        let mut result: Option<T> = None;

        // Visited positions hash set
        let mut visited = HashSet::new();
        visited.insert(start.clone());

        // Work stack
        let mut work_queue = VecDeque::new();
        work_queue.push_back(workitem(start.clone(), None));

        // Get next work item
        while let Some(cur_pos) = work_queue.pop_front() {
            // Call state callback
            if !state_cb(&cur_pos, &work_queue, &visited, &mut result)? {
                break;
            }

            // Get neigbouring positions which match criteria
            for n in self.neighbours(&cur_pos.pos, &neigh_chk, &visited) {
                // Insert in to the visited hash map
                visited.insert(n.clone());

                // Create new work queue entry
                work_queue.push_back(workitem(n, Some(&cur_pos)));
            }
        }

        Ok(result)
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
}
