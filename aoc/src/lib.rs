use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use memmap2::Mmap;

/// Parse an input file to a vector with a given transform
pub fn parse_input_vec<T, F>(day: usize, tfn: F) -> Result<Vec<T>, Box<dyn Error>>
where
    F: FnMut(String) -> T,
{
    let input = Input::new(day)?;
    parse_buf_vec(input.lines(), tfn)
}

/// Parse an input string to a vector with a given transform
pub fn parse_test_vec<T, F>(test: &str, tfn: F) -> Result<Vec<T>, Box<dyn Error>>
where
    F: FnMut(String) -> T,
{
    let buf = BufReader::new(test.as_bytes());
    parse_buf_vec(buf.lines(), tfn)
}

/// Memory mapped input
struct Input {
    mmap: Mmap,
}

impl Input {
    fn new(day: usize) -> Result<Self, Box<dyn Error>> {
        // Open the file
        let file = File::open(format!("inputs/day{:02}.txt", day))?;

        // Memory map it
        let mmap = unsafe { Mmap::map(&file)? };

        Ok(Self { mmap })
    }

    fn lines(&self) -> Lines<BufReader<&[u8]>> {
        let buf_reader = BufReader::new(self.mmap.as_ref());

        buf_reader.lines()
    }
}

/// Parse a lines iterator to a vector with a given transform
fn parse_buf_vec<T, F>(lines: Lines<BufReader<&[u8]>>, mut tfn: F) -> Result<Vec<T>, Box<dyn Error>>
where
    F: FnMut(String) -> T,
{
    let mut result = Vec::new();

    for l in lines {
        let line = l?;

        result.push(tfn(line));
    }

    Ok(result)
}
