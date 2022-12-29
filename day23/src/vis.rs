use std::error::Error;

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
    vis(minx, miny, maxx, maxy, elves2)?;

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

const COLOURS: usize = 20;
const COLOUR_PALETTE: [[u8; 3]; COLOURS + 1] = [
    [0x00, 0x00, 0x00],
    [0xe7, 0x1d, 0x43],
    [0xff, 0x00, 0x00],
    [0xff, 0x37, 0x00],
    [0xff, 0x6e, 0x00],
    [0xff, 0xa5, 0x00],
    [0xff, 0xc3, 0x00],
    [0xff, 0xe1, 0x00],
    [0xff, 0xff, 0x00],
    [0xaa, 0xd5, 0x00],
    [0x55, 0xaa, 0x00],
    [0x00, 0x80, 0x00],
    [0x00, 0x55, 0x55],
    [0x00, 0x2b, 0xaa],
    [0x00, 0x00, 0xff],
    [0x19, 0x00, 0xd5],
    [0x32, 0x00, 0xac],
    [0x4b, 0x00, 0x82],
    [0x81, 0x2b, 0xa6],
    [0xb8, 0x57, 0xca],
    [0xd0, 0x3a, 0x87],
];

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

        let colour_split = elves.len() as f64 / (COLOURS - 1) as f64;

        // Sort elves by age
        let mut ages = elves
            .iter()
            .enumerate()
            .map(|(i, e)| (elves.rounds() - e.last_move_round, i))
            .collect::<Vec<_>>();

        ages.sort();

        let mut last_age = 0;
        let mut colour = 1;
        let mut count = 0;

        for (age, i) in ages {
            if age != last_age {
                last_age = age;
                if count as f64 > colour_split {
                    count = 0;
                    colour += 1;
                }
            }

            let elf = elves.get_elf(i);

            frame[(elf.y - miny) as usize][(elf.x - minx) as usize] = colour;

            count += 1;
        }

        // let age_max = elves
        //     .iter()
        //     .map(|e| elves.rounds() - e.last_move_round)
        //     .max()
        //     .unwrap();

        // let age_step = age_max as f64 / (COLOURS - 1) as f64;

        // for e in elves.iter() {
        //     let move_age = if age_step == 0f64 {
        //         0f64
        //     } else {
        //         (elves.rounds() - e.last_move_round) as f64 / age_step
        //     };
        //     let colour = move_age as u8 + 1;
        //     assert!(colour >= 1 && colour <= COLOURS as u8);
        //     frame[(e.y - miny) as usize][(e.x - minx) as usize] = colour;
        // }

        gif.draw_frame(frame, 2)?;

        Ok(())
    };

    loop {
        draw_frame(&mut gif, &elves)?;

        if elves.move_all() == 0 {
            draw_frame(&mut gif, &elves)?;
            gif.delay(250)?;

            break;
        }
    }

    Ok(())
}
