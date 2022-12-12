use std::{collections::HashSet, error::Error};

use aoc::input::parse_input_line;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_line(6, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[char]) -> usize {
    find_unique(input, 4)
}

fn part2(input: &[char]) -> usize {
    find_unique(input, 14)
}

fn find_unique(input: &[char], window: usize) -> usize {
    input
        .windows(window)
        .enumerate()
        .find(|(_, elems)| HashSet::<char>::from_iter(elems.iter().copied()).len() == window)
        .unwrap()
        .0
        + window
}

// Input parsing

fn input_transform(line: String) -> Vec<char> {
    line.chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test1() {
        let input: Vec<char> = EXAMPLE1.chars().collect();
        assert_eq!(part1(&input), 7);
        assert_eq!(part2(&input), 19);
    }

    #[test]
    fn test2() {
        let input: Vec<char> = EXAMPLE2.chars().collect();
        assert_eq!(part1(&input), 5);
        assert_eq!(part2(&input), 23);
    }

    #[test]
    fn test3() {
        let input: Vec<char> = EXAMPLE3.chars().collect();
        assert_eq!(part1(&input), 6);
        assert_eq!(part2(&input), 23);
    }

    #[test]
    fn test4() {
        let input: Vec<char> = EXAMPLE4.chars().collect();
        assert_eq!(part1(&input), 10);
        assert_eq!(part2(&input), 29);
    }

    #[test]
    fn test5() {
        let input: Vec<char> = EXAMPLE5.chars().collect();
        assert_eq!(part1(&input), 11);
        assert_eq!(part2(&input), 26);
    }
}
