use lazy_static::lazy_static;
use prng_mt::mt19937::MT19937;
use std::cmp::{max, min};

use aoc::gif::{Gif, IdenticalAction};

use crate::InputEnt;

const SAND_X: u16 = 500;

const BORDER: u16 = 2;
const SCALE: u16 = 3;
const FRAME_DELAY: u16 = 2;
const FINAL_FRAME_DELAY: u16 = 1000;

const ROCK_COLOUR: u8 = 0;
const SAND_COLOUR: u8 = 1;
const BG_COLOURS: usize = 16;
const RND_RANGE: u8 = 16;

lazy_static! {
    /// GIF colour palette
    pub static ref PALETTE: Vec<[u8; 3]> = {
        let mut mt = MT19937::new(42);

        let mut rnd_adj = |val| {
            let adj = (mt.next() % RND_RANGE as u32) as isize - (RND_RANGE / 2) as isize;
            (val as isize + adj) as u8
        };

        vec![
            [122, 94, 91]/*210, 105, 30]*/,  // Rock (Cocoa brown)
            [237, 201, 175], // Sand (Desert sand)
        ].into_iter().chain((0..BG_COLOURS)
        .map(|_| {
            [rnd_adj(62), rnd_adj(34),rnd_adj(21)] // Background
        })).collect()
    };
}

pub struct Map {
    width: u16,
    height: u16,
    x_offset: u16,
    content: Vec<Vec<u8>>,
    anim_gif: Option<Gif>,
}

impl Map {
    /// Create new map, optionall with a floor
    pub fn new(input: &[InputEnt], file: Option<String>, floor: bool) -> Self {
        // Work out min x, max x and max y (min y is 0)
        let (mut min_x, mut max_x, mut max_y) =
            input
                .iter()
                .fold((u16::MAX, 0, 0), |(min_x, max_x, max_y), l| {
                    (
                        min(min_x, *l.iter().map(|(x, _)| x).min().unwrap()),
                        max(max_x, *l.iter().map(|(x, _)| x).max().unwrap()),
                        max(max_y, *l.iter().map(|(_, y)| y).max().unwrap()),
                    )
                });

        // Adjust for when adding the floor
        if floor {
            max_y += 2;
            min_x = SAND_X - max_y;
            max_x = SAND_X + max_y;
        };

        // Work out dimensions
        let height = max_y + BORDER;
        let width = (max_x - min_x) + (BORDER * 2);

        let x_offset = min_x - BORDER;

        // Create map content vector of vectors
        let mut mt = MT19937::new(42);
        let content = (0..height)
            .map(|_| {
                (0..width)
                    .map(|_| 2 + (mt.next() % BG_COLOURS as u32) as u8)
                    .collect()
            })
            .collect();

        let anim_gif = file.map(|file| {
            Gif::new(file.as_str(), &PALETTE, width, height, SCALE, SCALE)
                .expect("Failed to create animated gif")
        });

        // Create the struct
        let mut result = Self {
            width,
            height,
            x_offset,
            content,
            anim_gif,
        };

        // Draw the rock lines
        for l in input {
            let (mut cur_x, mut cur_y) = l[0];

            for (next_x, next_y) in l.iter().skip(1) {
                result.line(cur_x, cur_y, *next_x, *next_y);
                (cur_x, cur_y) = (*next_x, *next_y);
            }
        }

        // Add the floor
        if floor {
            result.line(min_x, max_y, max_x, max_y)
        }

        result
    }

    /// Drops a particle of sand in to the map
    pub fn drop_sand(&mut self) -> bool {
        let mut x = SAND_X;
        let mut y = 0;
        let mut path = Vec::with_capacity(self.height as usize);
        let mut last_down = true;

        if self.get_pixel(x, y) == SAND_COLOUR {
            return false;
        }

        while y < self.height - 1 {
            path.push((x, y));

            if self.pixel_is_background(x, y + 1) {
                // Go down
                y += 1;
                last_down = true;
            } else if self.pixel_is_background(x - 1, y + 1) {
                // Go down and left
                y += 1;
                x -= 1;
                last_down = false;
            } else if self.pixel_is_background(x + 1, y + 1) {
                // Go down and right
                y += 1;
                x += 1;
                last_down = false;
            } else {
                // Come to rest
                self.set_pixel(x, y, SAND_COLOUR);

                // Draw an animation frame?
                if last_down
                    && !self.pixel_is_background(x - 1, y + 1)
                    && !self.pixel_is_background(x + 1, y + 1)
                {
                    match &mut self.anim_gif {
                        Some(gif) => {
                            // Clone the map content
                            let mut frame = self.content.clone();

                            // Draw the path
                            for (x, y) in path {
                                Self::set_frame_pixel(&mut frame, self.x_offset, x, y, SAND_COLOUR)
                            }

                            // Draw the frame
                            gif.draw_frame(frame, FRAME_DELAY)
                                .expect("Error writing gif frame")
                        }
                        None => (),
                    }
                }

                return true;
            }
        }

        false
    }

    /// Get pixel at coordinate
    fn get_pixel(&self, x: u16, y: u16) -> u8 {
        self.content[y as usize][(x - self.x_offset) as usize]
    }

    /// Return true if the pixel at coordinate is the background
    fn pixel_is_background(&self, x: u16, y: u16) -> bool {
        !matches!(self.get_pixel(x, y), SAND_COLOUR | ROCK_COLOUR)
    }

    /// Draw a horizontal or vertical line
    fn line(&mut self, x1: u16, y1: u16, x2: u16, y2: u16) {
        let min_x = min(x1, x2);
        let max_x = max(x1, x2);
        let min_y = min(y1, y2);
        let max_y = max(y1, y2);

        if min_x == max_x {
            for y in (min_y)..=(max_y) {
                self.set_pixel(min_x, y, ROCK_COLOUR);
            }
        } else if min_y == max_y {
            for x in (min_x)..=(max_x) {
                self.set_pixel(x, min_y, ROCK_COLOUR);
            }
        } else {
            panic!("Not horizontal or vertical")
        }
    }

    /// Sets the pixel at a coordinate
    fn set_pixel(&mut self, x: u16, y: u16, colour: u8) {
        Self::set_frame_pixel(&mut self.content, self.x_offset, x, y, colour);
    }

    /// Sets the pixel in a frame buffer
    fn set_frame_pixel(frame: &mut [Vec<u8>], x_offset: u16, x: u16, y: u16, colour: u8) {
        frame[y as usize][(x - x_offset) as usize] = colour;
    }

    /// Draws the current state to a gif
    pub fn draw(&self, file: &str) {
        let mut gif = Gif::new(file, &PALETTE, self.width, self.height, SCALE, SCALE)
            .expect("Unable to create gif");

        gif.draw_frame(self.content.clone(), FRAME_DELAY)
            .expect("Failed to draw frame");
    }
}

impl Drop for Map {
    fn drop(&mut self) {
        // Animating?
        match &mut self.anim_gif {
            Some(gif) => {
                // Draw the final frame
                gif.draw_frame_identical_check(
                    self.content.clone(),
                    FINAL_FRAME_DELAY,
                    IdenticalAction::Delay,
                )
                .expect("Error writing gif frame")
            }
            None => (),
        }
    }
}
