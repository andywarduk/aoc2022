use std::error::Error;

use aoc::input::parse_input_vec;
use day14lib::{input_transform, DropResult, InputEnt, Map};

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(14, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> usize {
    do_part(input, false)
}

fn part2(input: &[InputEnt]) -> usize {
    do_part(input, true)
}

fn do_part(input: &[InputEnt], floor: bool) -> usize {
    // Create the map
    let mut map = Map::new(input, floor);

    // Drop sand counting how many fall
    let mut count = 0;

    while matches!(map.drop_sand(), DropResult::Rest(_)) {
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 24);
        assert_eq!(part2(&input), 93);
    }
}
