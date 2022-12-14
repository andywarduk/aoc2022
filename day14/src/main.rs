use std::error::Error;

use aoc::input::parse_input_vec;
use map::Map;

mod map;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(14, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input, Some("day14-1")));
    println!("Part 2: {}", part2(&input, Some("day14-2")));

    Ok(())
}

fn part1(input: &[InputEnt], file_stub: Option<&str>) -> u64 {
    do_part(input, file_stub, false)
}

fn part2(input: &[InputEnt], file_stub: Option<&str>) -> u64 {
    do_part(input, file_stub, true)
}

fn do_part(input: &[InputEnt], file_stub: Option<&str>, floor: bool) -> u64 {
    let anim_file = file_stub.map(|stub| format!("vis/{stub}-anim.gif"));

    // Create the map
    let mut map = Map::new(input, anim_file, floor);

    let mut count = 0;

    // Drop sand
    while map.drop_sand() {
        count += 1;
    }

    // Draw the final map
    if let Some(stub) = file_stub {
        map.draw(&format!("vis/{stub}-final.gif"));
    }

    count
}

// Input parsing

type InputEnt = Vec<(u16, u16)>;

fn input_transform(line: String) -> InputEnt {
    line.split("->")
        .map(|seg| {
            let coord: Vec<_> = seg
                .trim()
                .split(',')
                .map(|c| c.parse::<u16>().unwrap())
                .collect();

            (coord[0], coord[1])
        })
        .collect()
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
        assert_eq!(part1(&input, None), 24);
        assert_eq!(part2(&input, None), 93);
    }
}
