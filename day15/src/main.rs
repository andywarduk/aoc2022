use std::{
    cmp::{max, min},
    collections::HashSet,
    error::Error,
    str::SplitWhitespace,
};

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
    let occupied = get_occupied(sensors, row);
    let ranges = get_ranges(sensors, row);

    ranges
        .into_iter()
        .map(|(sx, ex)| (sx..=ex).filter(|x| !occupied.contains(x)).count())
        .sum()
}

fn part2(sensors: &[InputEnt], end_row: i32) -> i64 {
    for row in 0..=end_row {
        let ranges = get_ranges(sensors, row);

        match ranges.len() {
            1 => (),
            2 => {
                if ranges[0].1 + 2 != ranges[1].0 {
                    panic!("Gap is not 1");
                }

                return ((ranges[0].1 + 1) as i64 * 4_000_000) + row as i64;
            }
            _ => panic!("Ranges unexpected on row {row}: {ranges:?}"),
        }
    }

    panic!("Beacon position not found");
}

fn get_occupied(sensors: &[InputEnt], row: i32) -> HashSet<i32> {
    let mut occupied = HashSet::new();

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
        let intersect = s.dist - (row - s.y).abs();

        if intersect >= 0 {
            let mut sx = s.x - intersect;
            let mut ex = s.x + intersect;
            let mut add = true;

            ranges.retain(|(esx, eex)| {
                if sx >= *esx && ex <= *eex {
                    add = false;
                    return true;
                }

                if *esx >= sx && *esx <= ex
                    || *eex >= sx && *eex <= ex
                    || *esx == ex + 1
                    || *eex + 1 == sx
                {
                    sx = min(sx, *esx);
                    ex = max(ex, *eex);
                    false
                } else {
                    true
                }
            });

            if add {
                ranges.push((sx, ex));
            }

            ranges.sort();
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
    let mut iter = line.split_whitespace();

    let parse = |iter: &mut SplitWhitespace| {
        iter.next()
            .unwrap()
            .split('=')
            .nth(1)
            .unwrap()
            .split(',')
            .next()
            .unwrap()
            .split(':')
            .next()
            .unwrap()
            .parse::<i32>()
            .unwrap()
    };

    iter.next().unwrap();
    iter.next().unwrap();
    let x = parse(&mut iter);
    let y = parse(&mut iter);
    iter.next().unwrap();
    iter.next().unwrap();
    iter.next().unwrap();
    iter.next().unwrap();
    let cx = parse(&mut iter);
    let cy = parse(&mut iter);

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
