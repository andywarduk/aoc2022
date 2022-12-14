use std::error::Error;
use std::io::{stdout, Write};

use aoc::input::parse_input_vec;

use crate::map::Map;

mod map;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let map = Map::new(parse_input_vec(12, input_transform)?);

    // Run parts
    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));

    // Visualisations
    print!("Generating visualisations...");
    stdout().flush()?;

    part1vis(&map)?;
    print!(" part 1");
    stdout().flush()?;

    part2vis(&map)?;
    println!(" part2");

    Ok(())
}

/// Run part 1
fn part1(map: &Map) -> usize {
    // Shortest path from START to END, allowed to go up by 1 only
    map.shortest_path(map.start(), |n| *n == *map.end(), |from, to| to <= from + 1)
}

/// Run part 2
fn part2(map: &Map) -> usize {
    // Shortest path from END to height 0, allowed to go down by 1 only
    map.shortest_path(map.end(), |n| map.height(n) == 0, |from, to| to >= from - 1)
}

/// Scaling for GIF output
const GIF_SCALE: u16 = 6;

/// Generate visualisation for part 1
fn part1vis(map: &Map) -> Result<(), Box<dyn Error>> {
    // Shortest path from START to END, allowed to go up by 1 only
    map.shortest_path_vis(
        "vis/day12-1-anim.gif",
        "vis/day12-1-final.gif",
        GIF_SCALE,
        map.start(),
        |n| *n == *map.end(),
        |from, to| to <= from + 1,
    )
}

/// Generate visualisation for part 2
fn part2vis(map: &Map) -> Result<(), Box<dyn Error>> {
    // Shortest path from END to height 0, allowed to go down by 1 only
    map.shortest_path_vis(
        "vis/day12-2-anim.gif",
        "vis/day12-2-final.gif",
        GIF_SCALE,
        map.end(),
        |n| map.height(n) == 0,
        |from, to| to >= from - 1,
    )
}

/// Input parsing (no-op)
fn input_transform(line: String) -> String {
    line
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test1() {
        let input = Map::new(parse_test_vec(EXAMPLE1, input_transform).unwrap());
        assert_eq!(part1(&input), 31);
        assert_eq!(part2(&input), 29);
    }
}
