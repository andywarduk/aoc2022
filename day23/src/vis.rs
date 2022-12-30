use std::error::Error;

use hsl::HSL;
use lazy_static::lazy_static;

use aoc::{gif::Gif, input::parse_input_vec};

use day23lib::{elves::Elves, input::input_transform};

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(23, input_transform)?;

    let elves1 = Elves::build(input);
    let elves2 = elves1.clone();

    // Get bounding box
    println!("Calculating bounding box...");
    let (minx, miny, maxx, maxy) = get_bbox(elves1);

    // Generate visualisation
    println!("Generating visualisation...");
    vis(minx - 1, miny - 1, maxx + 1, maxy + 1, elves2)?;

    Ok(())
}

fn get_bbox(mut elves: Elves) -> (isize, isize, isize, isize) {
    loop {
        if elves.move_all() == 0 {
            break;
        }
    }

    elves.bbox()
}

const COLOURS: usize = 240;
const DELAY: u16 = 5;
const FINAL_DELAY: u16 = 1000;

lazy_static! {
    /// GIF colour palette
    pub static ref COLOUR_PALETTE: Vec<[u8; 3]> = {
        // Add black first
        [[0x00, 0x00, 0x00]].into_iter()
            .chain(
                // Rainbow colours, red to blue
                (0..COLOURS)
                    .map(|h| {
                        let hsl = HSL {
                            h: h as f64,
                            s: 1_f64,
                            l: 0.5_f64,
                        };

                        let (r,g,b) = hsl.to_rgb();

                        [r, g, b]
                    })
            )
            .chain([[0xff, 0x00, 0xff]].into_iter()) // Magenta on the end
            .collect::<Vec<_>>()
    };
}

fn vis(
    minx: isize,
    miny: isize,
    maxx: isize,
    maxy: isize,
    mut elves: Elves,
) -> Result<(), Box<dyn Error>> {
    let width = (maxx - minx) + 1;
    let height = (maxy - miny) + 1;

    let mut gif = Gif::new(
        "vis/day23-anim.gif",
        &COLOUR_PALETTE,
        width as u16,
        height as u16,
        4,
        4,
    )?;

    let draw_frame = |gif: &mut Gif, elves: &Elves| -> Result<(), Box<dyn Error>> {
        let mut frame = vec![vec![0; width as usize]; height as usize];

        let colour_split = elves.len() as f64 / COLOURS as f64;

        // Sort elves by rounds since last move
        let mut ages = elves
            .iter()
            .enumerate()
            .map(|(i, e)| {
                (
                    match e.last_move_round {
                        None => usize::MAX,
                        Some(last) => elves.rounds() - last,
                    },
                    i,
                )
            })
            .collect::<Vec<_>>();

        ages.sort();

        // Render loop variables
        let mut last_age = 0;
        let mut colour = 1;
        let mut count = 0f64;

        // Render in age ascending order
        for (age, i) in ages {
            if age != last_age {
                // Age has changed
                if age == usize::MAX {
                    // Not moved at all yet - clamp to last colour
                    colour = COLOURS as u8;
                } else {
                    // Work out colour for the next split
                    while count > colour_split {
                        count -= colour_split;
                        colour += 1;
                    }
                }

                // Update last age
                last_age = age;
            }

            // Get the elf
            let elf = elves.get_elf(i);

            // Draw as a point
            frame[(elf.y - miny) as usize][(elf.x - minx) as usize] = colour;

            // Increment count
            count += 1f64;
        }

        // Render the frame
        gif.draw_frame(frame, DELAY)?;

        Ok(())
    };

    loop {
        // Draw frame
        draw_frame(&mut gif, &elves)?;

        // Move the elves
        if elves.move_all() == 0 {
            // Final frame
            draw_frame(&mut gif, &elves)?;

            // Delay
            gif.delay(FINAL_DELAY)?;

            break;
        }
    }

    Ok(())
}
