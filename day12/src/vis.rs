use std::{
    borrow::Cow,
    cmp::{max, min},
    collections::{HashSet, VecDeque},
    error::Error,
    fs::File,
};

use gif::{Encoder, Frame, Repeat};
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
    static ref COLOUR_MAP: Vec<u8> = {(0..=3)
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
        // Calculate gif dimensions
        let gif_width = (self.max_x + 1) * GIF_SCALE;
        let gif_height = (self.max_y + 1) * GIF_SCALE;

        // Create the animated output file
        let mut image = File::create(path_anim)?;

        // Create the gif encoder
        let mut encoder = Encoder::new(&mut image, gif_width, gif_height, &COLOUR_MAP)?;

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
        let state_cb = |cur_pos: &WorkItem<Vec<Pos>>,
                        work_queue: &VecDeque<WorkItem<Vec<Pos>>>,
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
        let mut encoder = Encoder::new(&mut image, gif_width, gif_height, &COLOUR_MAP)?;

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
