use std::error::Error;
use std::io::{stdout, Write};

use aoc::input::parse_input_vec;

use day12lib::Map;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let map = Map::new(parse_input_vec(12, input_transform)?);

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
