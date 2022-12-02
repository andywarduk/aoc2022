use std::{
    error::Error,
    io::{BufReader, Lines},
};

use aoc::Input;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = get_input($day)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    0 // TODO
}

fn part2(input: &[InputEnt]) -> u64 {
    0 // TODO
}

// Input parsing

type InputEnt = String; // TODO

fn get_input(day: usize) -> Result<Vec<InputEnt>, Box<dyn Error>> {
    let input = Input::new(day)?;

    parse_input(input.lines())
}

fn parse_input(lines: Lines<BufReader<&[u8]>>) -> Result<Vec<InputEnt>, Box<dyn Error>> {
    let mut result = Vec::new();

    for l in lines {
        let line = l?;

        // TODO process line to InputEnt
        let ent = line;

        result.push(ent);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use aoc::test_input;

    use super::*;

    const EXAMPLE1: &str = "TODO";

    #[test]
    fn test1() {
        let input = parse_input(test_input(EXAMPLE1)).unwrap();
        assert_eq!(part1(&input), 0 /* TODO */);
        assert_eq!(part2(&input), 0 /* TODO */);
    }
}
