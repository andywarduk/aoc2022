use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use memmap2::Mmap;

pub struct Input {
    mmap: Mmap,
}

impl Input {
    pub fn new(day: usize) -> Result<Self, Box<dyn Error>> {
        // Open the file
        let file = File::open(format!("inputs/day{:02}.txt", day))?;

        // Memory map it
        let mmap = unsafe { Mmap::map(&file)? };

        Ok(Self { mmap })
    }

    pub fn lines(&self) -> Lines<BufReader<&[u8]>> {
        let buf_reader = BufReader::new(self.mmap.as_ref());

        buf_reader.lines()
    }
}
