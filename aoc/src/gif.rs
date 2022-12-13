use std::{
    borrow::Cow,
    cmp::{max, min},
    error::Error,
    fs::File,
};

use gif::{Encoder, Frame, Repeat};

pub struct Gif {
    width: u16,
    height: u16,
    x_scale: u16,
    y_scale: u16,
    gif_width: u16,
    gif_height: u16,
    encoder: Encoder<File>,
    last_frame: Option<Vec<Vec<u8>>>,
}

impl Gif {
    pub fn new(
        file: &str,
        palette: &[[u8; 3]],
        width: u16,
        height: u16,
        x_scale: u16,
        y_scale: u16,
    ) -> Result<Self, Box<dyn Error>> {
        let gif_width = width * x_scale;
        let gif_height = height * y_scale;

        // Create the flattened palette
        let flat_pal = palette.iter().flatten().cloned().collect::<Vec<_>>();

        // Create the encoder
        let mut encoder = Encoder::new(File::create(file)?, gif_width, gif_height, &flat_pal)?;

        // Ininitely repeat
        encoder.set_repeat(Repeat::Infinite)?;

        Ok(Self {
            width,
            height,
            x_scale,
            y_scale,
            gif_width,
            gif_height,
            encoder,
            last_frame: None,
        })
    }

    pub fn draw_frame(
        &mut self,
        frame_data: Vec<Vec<u8>>,
        delay: u16,
    ) -> Result<(), Box<dyn Error>> {
        // Make sure the frame looks like the correct size
        assert_eq!(frame_data.len(), self.height as usize);
        assert_eq!(frame_data[0].len(), self.width as usize);

        let mut min_x;
        let mut max_x;
        let mut min_y;
        let mut max_y;

        if let Some(last_frame) = &self.last_frame {
            // Work out difference between this frame and the last
            min_x = usize::MAX;
            max_x = 0;
            min_y = usize::MAX;
            max_y = 0;

            for (y, (l1, l2)) in last_frame.iter().zip(frame_data.iter()).enumerate() {
                for (x, (p1, p2)) in l1.iter().zip(l2.iter()).enumerate() {
                    if p1 != p2 {
                        min_x = min(min_x, x);
                        max_x = max(max_x, x);
                        min_y = min(min_y, y);
                        max_y = max(max_y, y);
                    }
                }
            }
        } else {
            // Draw whole frame
            min_x = 0;
            max_x = self.width as usize - 1;
            min_y = 0;
            max_y = self.height as usize - 1;
        }

        // Extract frame portion
        let frame_section: Vec<&[u8]> = frame_data
            .iter()
            .enumerate()
            .filter_map(|(y, l)| {
                if y >= min_y && y <= max_y {
                    Some(&l[min_x..=max_x])
                } else {
                    None
                }
            })
            .collect();

        // Scale the frame up
        let out_section = frame_section.into_iter().fold(
            Vec::with_capacity(self.gif_height as usize * self.gif_width as usize),
            |mut acc: Vec<u8>, line| {
                let expanded_line: Vec<u8> = line
                    .iter()
                    .flat_map(|pix| vec![*pix; self.x_scale as usize])
                    .collect();

                for _ in 0..self.y_scale {
                    acc.extend(&expanded_line);
                }

                acc
            },
        );

        let frame = Frame {
            top: min_y as u16 * self.y_scale,
            left: min_x as u16 * self.x_scale,
            width: ((max_x - min_x) as u16 + 1) * self.x_scale,
            height: ((max_y - min_y) as u16 + 1) * self.y_scale,
            buffer: Cow::Borrowed(&*out_section),
            delay: max(2, delay),
            ..Default::default()
        };

        self.encoder.write_frame(&frame)?;

        self.last_frame = Some(frame_data);

        Ok(())
    }
}
