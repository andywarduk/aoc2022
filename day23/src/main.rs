use std::error::Error;

use aoc::input::parse_input_vec;

use day23lib::{elves::Elves, input::input_transform};

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(23, input_transform)?;

    let elves1 = Elves::build(input);
    let elves2 = elves1.clone();

    // Run parts
    println!("Part 1: {}", part1(elves1));
    println!("Part 2: {}", part2(elves2));

    Ok(())
}

fn part1(mut elves: Elves) -> usize {
    // Run 10 rounds
    for _ in 0..10 {
        elves.move_all();
    }

    // Get bounding box
    let (minx, miny, maxx, maxy) = elves.bbox();

    // Calculate area
    let area = (((maxx - minx) + 1) * ((maxy - miny) + 1)) as usize;

    // Calculate empty squares
    area - elves.len()
}

fn part2(mut elves: Elves) -> usize {
    // Run until no elves move
    loop {
        if elves.move_all() == 0 {
            break elves.rounds();
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "##
#.
..
##
";

    const EXAMPLE2: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let mut elves = Elves::build(input);

        elves.move_all();
        println!("{:?}", elves);
        assert_eq!(format!("{:?}", elves), "##\n..\n#.\n.#\n#.\n");
        elves.move_all();
        println!("{:?}", elves);
        assert_eq!(format!("{:?}", elves), ".##.\n#...\n...#\n....\n.#..\n");
        elves.move_all();
        println!("{:?}", elves);
        assert_eq!(
            format!("{:?}", elves),
            "..#..\n....#\n#....\n....#\n.....\n..#..\n"
        );
        elves.move_all();
        println!("{:?}", elves);
        assert_eq!(
            format!("{:?}", elves),
            "..#..\n....#\n#....\n....#\n.....\n..#..\n"
        );
    }

    #[test]
    fn test2() {
        let input = parse_test_vec(EXAMPLE2, input_transform).unwrap();
        let elves = Elves::build(input);

        assert_eq!(part1(elves.clone()), 110);
        assert_eq!(part2(elves), 20);
    }
}
