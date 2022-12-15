use std::{
    cmp::{max, min},
    collections::HashSet,
    error::Error,
};

use lazy_static::lazy_static;
use regex::Regex;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(15, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input, 2_000_000));
    println!("Part 2: {}", part2(&input, 4_000_000));

    Ok(())
}

fn part1(sensors: &[InputEnt], row: i32) -> usize {
    // Find sensors and beacon occupying this row
    let occupied = get_occupied(sensors, row);

    // Get scan range for the row
    let ranges = get_ranges(sensors, row);

    // Return number of locations scanned minus locations occupied by sensors and beacons
    ranges
        .into_iter()
        .map(|(sx, ex)| (sx..=ex).filter(|x| !occupied.contains(x)).count())
        .sum()
}

fn part2(sensors: &[InputEnt], end_row: i32) -> i64 {
    // Scan row range
    (0..=end_row)
        .find_map(|row| {
            // Get scan range(s) for this row
            let ranges = get_ranges(sensors, row);

            match ranges.len() {
                1 => None, // Single range
                2 => {
                    // Found a gap
                    assert!(ranges[0].1 + 2 == ranges[1].0, "Gap should be 1");
                    Some(((ranges[0].1 + 1) as i64 * 4_000_000) + row as i64)
                }
                _ => panic!("Ranges unexpected on row {row}: {ranges:?}"),
            }
        })
        .expect("Beacon position not found")
}

fn get_occupied(sensors: &[InputEnt], row: i32) -> HashSet<i32> {
    let mut occupied = HashSet::new();

    // Get occupied positions in this row
    for s in sensors {
        if s.y == row {
            occupied.insert(s.x);
        }

        if s.cy == row {
            occupied.insert(s.cx);
        }
    }

    occupied
}

fn get_ranges(sensors: &[InputEnt], row: i32) -> Vec<(i32, i32)> {
    let mut ranges = Vec::new();

    for s in sensors {
        // Find the intersection distance from the sensor to the row
        let intersect = s.dist - (row - s.y).abs();

        if intersect >= 0 {
            // Range does intersect this row
            // Work out the start and end x position if the intersection
            let mut sx = s.x - intersect;
            let mut ex = s.x + intersect;

            ranges.retain(|(esx, eex)| {
                // New start within this range or
                // New end withing this range or
                // This range right after new range or
                // New range right after this range or
                // New range contained in this range
                if *esx >= sx && *esx <= ex
                    || *eex >= sx && *eex <= ex
                    || *esx == ex + 1
                    || *eex + 1 == sx
                    || sx >= *esx && ex <= *eex
                {
                    // Calculate new combined range
                    sx = min(sx, *esx);
                    ex = max(ex, *eex);

                    // Don't retain this range
                    false
                } else {
                    // Retain this range
                    true
                }
            });

            // Add new range
            ranges.push((sx, ex));
        }
    }

    ranges
}

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    cx: i32,
    cy: i32,
    dist: i32,
}

// Input parsing

type InputEnt = Sensor;

fn input_transform(line: String) -> InputEnt {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^Sensor at x=(-?\d*), y=(-?\d*): closest beacon is at x=(-?\d*), y=(-?\d*)$"
        )
        .unwrap();
    }

    let nums: Vec<i32> = RE
        .captures(&line)
        .unwrap_or_else(|| panic!("Invalid input line: {line}"))
        .iter()
        .skip(1)
        .map(|m| {
            m.expect("No match")
                .as_str()
                .parse::<i32>()
                .expect("Invalid number")
        })
        .collect();

    let x = nums[0];
    let y = nums[1];
    let cx = nums[2];
    let cy = nums[3];

    let dx = (x - cx).abs();
    let dy = (y - cy).abs();
    let dist = dx + dy;

    Sensor { x, y, cx, cy, dist }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input, 10), 26);
        assert_eq!(part2(&input, 20), 56000011);
    }
}
