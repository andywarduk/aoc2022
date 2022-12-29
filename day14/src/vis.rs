use std::error::Error;

use lazy_static::lazy_static;
use prng_mt::mt19937::MT19937;

use aoc::gif::{Gif, IdenticalAction};
use aoc::input::parse_input_vec;

use day14lib::{input_transform, DropResult, InputEnt, Map, Tile};

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(14, input_transform)?;

    // Create visualisations
    println!("Generating visualisations...");
    do_part(&input, "day14-1", false);
    do_part(&input, "day14-2", true);

    Ok(())
}

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

fn do_part(input: &[InputEnt], file_stub: &str, floor: bool) {
    let anim_file = format!("vis/{file_stub}-anim.gif");

    // Create the map
    let mut map = Map::new(input, floor);

    // Create background base frame
    let mut mt = MT19937::new(42);
    let base_frame = (0..map.height)
        .map(|_| {
            (0..map.width)
                .map(|_| 2 + (mt.next() % BG_COLOURS as u32) as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Create the animated GIF
    let mut anim_gif = Gif::new(
        anim_file.as_str(),
        &PALETTE,
        map.width as u16,
        map.height as u16,
        SCALE,
        SCALE,
    )
    .expect("Failed to create animated gif");

    // Drop sand
    let mut last_path = vec![];

    loop {
        match map.drop_sand() {
            DropResult::Full => {
                // Draw the final frame
                draw_frame(
                    &mut anim_gif,
                    &map,
                    &vec![],
                    base_frame.clone(),
                    FINAL_FRAME_DELAY,
                    IdenticalAction::Delay,
                );

                break;
            }
            DropResult::Out(path) => {
                // Draw the final frame
                draw_frame(
                    &mut anim_gif,
                    &map,
                    &path,
                    base_frame.clone(),
                    FINAL_FRAME_DELAY,
                    IdenticalAction::Delay,
                );

                last_path = path.clone();

                break;
            }
            DropResult::Rest(path) => {
                // Draw an animation frame?

                // Was the last movement downwards?
                if path.len() >= 2 {
                    let (x, y) = path[path.len() - 1];
                    let (px, _) = path[path.len() - 2];

                    let last_down = x == px;

                    if last_down
                        && !map.tile_is_empty(x - 1, y + 1)
                        && !map.tile_is_empty(x + 1, y + 1)
                    {
                        draw_frame(
                            &mut anim_gif,
                            &map,
                            &path,
                            base_frame.clone(),
                            FRAME_DELAY,
                            IdenticalAction::Ignore,
                        );
                    }
                }
            }
        }
    }

    // Draw the final map
    draw(
        &map,
        base_frame,
        &last_path,
        format!("vis/{file_stub}-final.gif"),
    );
}

fn draw(map: &Map, frame_data: Vec<Vec<u8>>, path: &Vec<(usize, usize)>, file: String) {
    let mut gif = Gif::new(
        file.as_str(),
        &PALETTE,
        map.width as u16,
        map.height as u16,
        SCALE,
        SCALE,
    )
    .expect("Unable to create gif");

    draw_frame(
        &mut gif,
        map,
        path,
        frame_data,
        FRAME_DELAY,
        IdenticalAction::Ignore,
    );
}

fn draw_frame(
    gif: &mut Gif,
    map: &Map,
    path: &Vec<(usize, usize)>,
    mut frame_data: Vec<Vec<u8>>,
    delay: u16,
    identical_action: IdenticalAction,
) {
    // Draw tiles
    for (y, row) in map.content.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            match tile {
                Tile::Sand => frame_data[y][x] = SAND_COLOUR,
                Tile::Rock => frame_data[y][x] = ROCK_COLOUR,
                Tile::Empty => (),
            }
        }
    }

    // Draw path
    for (x, y) in path {
        frame_data[*y][*x - map.x_offset] = SAND_COLOUR;
    }

    // Output the frame
    gif.draw_frame_identical_check(frame_data, delay, identical_action)
        .expect("Failed to draw frame")
}
