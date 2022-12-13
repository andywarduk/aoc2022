use std::collections::{HashSet, VecDeque};
use std::error::Error;

use aoc::gif::Gif;
use lazy_static::lazy_static;

use crate::{Map, Pos, WorkItem};

const GIF_SCALE: u16 = 6;
const MIN_COLOUR_COMPONENT: u8 = 32;

pub fn part1vis(map: &Map) -> Result<(), Box<dyn Error>> {
    // Shortest path from START to END, allowed to go up by 1 only
    map.shortest_path_vis(
        "vis/day12-1-anim.gif",
        "vis/day12-1-final.gif",
        &map.start,
        |n| *n == map.end,
        |from, to| to <= from + 1,
    )
}

pub fn part2vis(map: &Map) -> Result<(), Box<dyn Error>> {
    // Shortest path from END to height 0, allowed to go down by 1 only
    map.shortest_path_vis(
        "vis/day12-2-anim.gif",
        "vis/day12-2-final.gif",
        &map.end,
        |n| map.height(n) == 0,
        |from, to| to >= from - 1,
    )
}

lazy_static! {
    /// GIF colour palette
    static ref COLOUR_MAP: Vec<[u8; 3]> = {(0..=3)
        .flat_map(|i| {
            (0..26)
                .map(|j| {
                    let val = MIN_COLOUR_COMPONENT + (((255 - MIN_COLOUR_COMPONENT) / 26) * j);

                    match i {
                        0 => [0, val, 0],   // Green
                        1 => [val, 0, 0],   // Red
                        2 => [0, 0, val],   // Blue
                        3 => [val, val, 0], // Yellow
                        _ => unreachable!(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
    };
}

impl Map {
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
        // Create the animated gif
        let mut gif = Gif::new(
            path_anim,
            &COLOUR_MAP,
            self.max_x + 1,
            self.max_y + 1,
            GIF_SCALE,
            GIF_SCALE,
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
                        visited: &HashSet<Pos>|
         -> Result<(), Box<dyn Error>> {
            // Only plot if path length has increased
            if cur_pos.data.len() == last_len {
                return Ok(());
            }
            last_len = cur_pos.data.len();

            let mut next_frame = map_frame.clone();

            // Colour visited blue
            for v in visited {
                colour_pixel(&mut next_frame, v.x, v.y, 2);
            }

            // Colour work queue red
            for v in work_queue {
                colour_pixel(&mut next_frame, v.pos.x, v.pos.y, 1);
            }

            // Colour current yellow
            for p in cur_pos.data.iter() {
                colour_pixel(&mut next_frame, p.x, p.y, 3)
            }

            // TODO colour_pixel(&mut next_frame, cur_pos.pos.x, cur_pos.pos.y, 3);

            // Draw the frame
            gif.draw_frame(next_frame, 2)?;

            Ok(())
        };

        // Get shortest path
        let path = self.shortest_path_internal(
            start,
            end_chk,
            neigh_chk,
            |n, w| {
                let mut path = w.data.clone();
                path.push(n.clone());
                WorkItem::new(n, path)
            },
            state_cb,
        )?;

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
            GIF_SCALE,
            GIF_SCALE,
        )?;

        gif.draw_frame(next_frame, 2)?;

        Ok(())
    }
}
