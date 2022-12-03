use std::error::Error;

use aoc::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(3, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    input.iter().fold(0, |tot, pack| {
        let split = pack.split_at(pack.len() / 2);
        tot + *split.0.iter().find(|item| split.1.contains(item)).unwrap() as u64
    })
}

fn part2(input: &[InputEnt]) -> u64 {
    input.chunks(3).fold(0, |tot, grp| {
        let mut first = grp[0].clone();
        first.sort();
        first.dedup();

        for pack in grp.iter().skip(1) {
            first.retain(|item| pack.contains(item));
        }

        assert!(first.len() == 1);

        tot + first[0] as u64
    })
}

// Input parsing

type InputEnt = Vec<u8>;

fn input_transform(line: String) -> InputEnt {
    line.chars()
        .map(|c| match c {
            'a'..='z' => c as u8 - b'a' + 1,
            'A'..='Z' => c as u8 - b'A' + 27,
            _ => panic!("Unexpected character {}", c),
        })
        .collect::<Vec<u8>>()
}

#[cfg(test)]
mod tests {
    use aoc::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 157);
        assert_eq!(part2(&input), 70);
    }
}
