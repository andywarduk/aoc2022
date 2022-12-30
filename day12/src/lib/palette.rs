use lab::Lab;
use lazy_static::lazy_static;

const COLOUR_STEP: u8 = 6;
const COLOUR_MAX: u8 = 255;
const MIN_COLOUR_COMPONENT: u8 = COLOUR_MAX - (26 * COLOUR_STEP);

lazy_static! {
    /// GIF colour palette
    pub static ref COLOUR_MAP: Vec<[u8; 3]> = {(0..=3)
        .flat_map(|i| {
            (0..26)
                .map(|j| {
                    let val = MIN_COLOUR_COMPONENT + (COLOUR_STEP * j);

                    match i {
                        0 => [0, val, 0], // Green (terrain)
                        1 => {
                            // Red (working)
                            let mut lab = Lab::from_rgb(&[0, val, 0]);
                            lab.a = -lab.a; // Green <-> red axis
                            lab.l += 5f32; // Lightness
                            lab.to_rgb()
                        }
                        2 => {
                            // Cyan (visited)
                            let mut lab = Lab::from_rgb(&[0, val, 0]);
                            lab.b = -(lab.b / 2f32); // Blue <-> yellow axis
                            lab.to_rgb()
                        }
                        3 => {
                            // Yellow (path)
                            let mut lab = Lab::from_rgb(&[0, val, 0]);
                            lab.a = 0f32; // Green <-> red axis
                            lab.b = 128f32; // Blue <-> yellow axis
                            lab.l += 15f32; // Lightness
                            lab.to_rgb()
                        }
                        _ => unreachable!(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
    };
}
