use std::error::Error;

use aoc::input::parse_input_vec;

use day24lib::input::{input_transform, InputEnt};
use day24lib::map::Map;
use day24lib::shortest_path::shortest_path;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(24, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> usize {
    let map = Map::new(input);

    shortest_path(
        &map,
        1,
        map.entry.clone(),
        map.exit.clone(),
        |_, _, _, _| {},
    )
    .unwrap()
}

fn part2(input: &[InputEnt]) -> usize {
    let map = Map::new(input);

    let time = shortest_path(
        &map,
        1,
        map.entry.clone(),
        map.exit.clone(),
        |_, _, _, _| {},
    )
    .unwrap();

    let time = shortest_path(
        &map,
        time,
        map.exit.clone(),
        map.entry.clone(),
        |_, _, _, _| {},
    )
    .unwrap();

    shortest_path(
        &map,
        time,
        map.entry.clone(),
        map.exit.clone(),
        |_, _, _, _| {},
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 18);
        assert_eq!(part2(&input), 54);
    }
}
