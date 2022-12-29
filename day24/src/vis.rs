use std::cmp::min;
use std::collections::{HashMap, VecDeque};
use std::error::Error;

use lazy_static::lazy_static;

use aoc::gif::Gif;
use aoc::input::parse_input_vec;

use day24lib::input::{input_transform, InputEnt};
use day24lib::map::Map;
use day24lib::pos::Pos;
use day24lib::shortest_path::{shortest_path, State};

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(24, input_transform)?;

    // Generate visualisation
    vis(&input, "vis/day24-anim.gif")?;

    Ok(())
}

fn vis(input: &[InputEnt], file: &str) -> Result<(), Box<dyn Error>> {
    let map = Map::new(input);

    // Create the animated gif
    let mut gif = Gif::new(
        file,
        &COLOUR_MAP,
        (map.width + BORDER + BORDER) as u16,
        (map.height + BORDER + BORDER) as u16,
        SCALE,
        SCALE,
    )?;

    let mut base_frame =
        vec![vec![BG_COLOUR; map.width + BORDER + BORDER]; map.height + BORDER + BORDER];

    (0..BORDER).for_each(|y| {
        for x in 0..(map.width + BORDER + BORDER) {
            base_frame[y][x] = WALL_COLOUR;
        }
    });

    ((map.height + BORDER)..(map.height + BORDER + BORDER)).for_each(|y| {
        for x in 0..(map.width + BORDER + BORDER) {
            base_frame[y][x] = WALL_COLOUR;
        }
    });

    (0..(map.height + BORDER + BORDER)).for_each(|y| {
        for x in 0..BORDER {
            base_frame[y][x] = WALL_COLOUR;
        }
    });

    (0..(map.height + BORDER + BORDER)).for_each(|y| {
        for x in (map.width + BORDER)..(map.width + BORDER + BORDER) {
            base_frame[y][x] = WALL_COLOUR;
        }
    });

    base_frame[map.entry.y + BORDER_OFF][map.entry.x + BORDER_OFF] = BG_COLOUR;
    base_frame[map.exit.y + BORDER_OFF][map.exit.x + BORDER_OFF] = BG_COLOUR;

    let mut last_time = 0;
    let mut paths: Vec<Vec<Pos>> = Vec::new();

    let mut state_cb = |state: &State,
                        blizzards: &HashMap<Pos, usize>,
                        states: &VecDeque<State>,
                        end_pos: &Pos| {
        if state.time != last_time {
            last_time = state.time;

            let mut frame = base_frame.clone();

            // Draw blizzards
            for (bpos, bcnt) in blizzards {
                frame[bpos.y + BORDER_OFF][bpos.x + BORDER_OFF] =
                    BLIZZARD_COL_BASE + (min(COLOUR_GRADES, *bcnt as u8) - 1);
            }

            // Draw work list
            for state in states {
                frame[state.pos.y + BORDER_OFF][state.pos.x + BORDER_OFF] = WORKING_COLOUR;
            }

            let (_, best_path) = states.iter().fold(
                (state.pos.dist(end_pos), &state.path),
                |(best_dist, best_path), state| {
                    let dist = state.pos.dist(end_pos);

                    if dist < best_dist {
                        (dist, &state.path)
                    } else {
                        (best_dist, best_path)
                    }
                },
            );

            // Draw previous paths
            for (i, path) in paths.iter().enumerate() {
                for p in path.iter() {
                    frame[p.y + BORDER_OFF][p.x + BORDER_OFF] = PATH_COLOUR_BASE + i as u8;
                }
            }

            // Draw current path
            for p in best_path {
                frame[p.y + BORDER_OFF][p.x + BORDER_OFF] = PATH_COLOUR;
            }

            gif.draw_frame(frame, 2).unwrap();
        }

        if state.pos == *end_pos {
            paths.push(state.path.clone());
            gif.delay(250).unwrap();
        }
    };

    let time = shortest_path(&map, 1, map.entry.clone(), map.exit.clone(), &mut state_cb).unwrap();

    let time = shortest_path(
        &map,
        time,
        map.exit.clone(),
        map.entry.clone(),
        &mut state_cb,
    )
    .unwrap();

    shortest_path(
        &map,
        time,
        map.entry.clone(),
        map.exit.clone(),
        &mut state_cb,
    )
    .unwrap();

    gif.delay(750).unwrap();

    Ok(())
}

const MIN_COLOUR_COMPONENT: u8 = 128;
const COLOUR_GRADES: u8 = 4;
const SCALE: u16 = 6;

const BORDER: usize = 2;
const BORDER_OFF: usize = BORDER - 1;

const BG_COLOUR: u8 = COLOUR_GRADES; // Black
const WALL_COLOUR: u8 = COLOUR_GRADES + 2;
const PATH_COLOUR_BASE: u8 = COLOUR_GRADES * 3; // Yellow base
const PATH_COLOUR: u8 = PATH_COLOUR_BASE + 3; // Bright yellow
const WORKING_COLOUR: u8 = (COLOUR_GRADES * 2) + 1; // Dull red
const BLIZZARD_COL_BASE: u8 = 0; // White/grey base

lazy_static! {
    /// GIF colour palette
    pub static ref COLOUR_MAP: Vec<[u8; 3]> = {
        (0..=3)
        .flat_map(|i| {
            (0..COLOUR_GRADES)
                .map(|j| {
                    let val = MIN_COLOUR_COMPONENT + (((255 - MIN_COLOUR_COMPONENT) / COLOUR_GRADES) * j);

                    match i {
                        0 => [val, val, val], // White (blizzards)
                        1 => [val - MIN_COLOUR_COMPONENT, val - MIN_COLOUR_COMPONENT, val - MIN_COLOUR_COMPONENT], // Black (walls)
                        2 => [val, 0, 0],     // Red (working)
                        3 => [val, val, 0],   // Yellow (path)
                        _ => unreachable!(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
    };
}
