use std::error::Error;

use aoc::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(4, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    input.iter().filter(|(r1, r2)| r1.contained(r2)).count() as u64
}

fn part2(input: &[InputEnt]) -> u64 {
    input.iter().filter(|(r1, r2)| r1.overlaps(r2)).count() as u64
}

struct Range {
    from: u8,
    to: u8,
}

impl Range {
    fn contained(&self, other: &Range) -> bool {
        (self.from <= other.from && self.to >= other.to)
            || (other.from <= self.from && other.to >= self.to)
    }

    fn overlaps(&self, other: &Range) -> bool {
        (self.from >= other.from && self.from <= other.to)
            || (self.to >= other.from && self.to <= other.to)
            || (other.from >= self.from && other.from <= self.to)
            || (other.to >= self.from && other.to <= self.to)
    }
}

impl From<&str> for Range {
    fn from(range_str: &str) -> Self {
        let split: Vec<&str> = range_str.split('-').collect();

        Range {
            from: split[0].parse::<u8>().unwrap(),
            to: split[1].parse::<u8>().unwrap(),
        }
    }
}

// Input parsing

type InputEnt = (Range, Range);

fn input_transform(line: String) -> InputEnt {
    let split: Vec<&str> = line.split(',').collect();

    (split[0].into(), split[1].into())
}

#[cfg(test)]
mod tests {
    use aoc::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 2);
        assert_eq!(part2(&input), 4);
    }
}
