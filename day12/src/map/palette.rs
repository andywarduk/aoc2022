use lazy_static::lazy_static;

const MIN_COLOUR_COMPONENT: u8 = 32;

lazy_static! {
    /// GIF colour palette
    pub static ref COLOUR_MAP: Vec<[u8; 3]> = {(0..=3)
        .flat_map(|i| {
            (0..26)
                .map(|j| {
                    let val1 = MIN_COLOUR_COMPONENT + (((255 - MIN_COLOUR_COMPONENT) / 26) * j);
                    let val2 = MIN_COLOUR_COMPONENT + (((255 - MIN_COLOUR_COMPONENT) / 39) * j);
                    let val3 = MIN_COLOUR_COMPONENT + (((255 - MIN_COLOUR_COMPONENT) / 52) * j);

                    match i {
                        0 => [0, val1, 0],    // Green (terrain)
                        1 => [val1, 0, 0],    // Red (working)
                        2 => [0, val2, val3], // Cyan (visited)
                        3 => [val1, val1, 0], // Yellow (path)
                        _ => unreachable!(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
    };
}
