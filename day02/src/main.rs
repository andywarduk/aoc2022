use std::{
    error::Error,
    io::{BufReader, Lines},
};

use aoc::Input;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = get_input(2)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> u64 {
    input
        .iter()
        .map(|(a, b, _)| b.score() + PlayResult::from_play(a, b).score())
        .sum()
}

fn part2(input: &[InputEnt]) -> u64 {
    input
        .iter()
        .map(|(a, _, res)| res.score() + a.play_for_result(res).score())
        .sum()
}

enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn score(&self) -> u64 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn play_for_result(&self, result: &PlayResult) -> Self {
        match (self, result) {
            (Play::Rock, PlayResult::Win) => Play::Paper,
            (Play::Rock, PlayResult::Lose) => Play::Scissors,
            (Play::Rock, PlayResult::Draw) => Play::Rock,
            (Play::Paper, PlayResult::Win) => Play::Scissors,
            (Play::Paper, PlayResult::Lose) => Play::Rock,
            (Play::Paper, PlayResult::Draw) => Play::Paper,
            (Play::Scissors, PlayResult::Win) => Play::Rock,
            (Play::Scissors, PlayResult::Lose) => Play::Paper,
            (Play::Scissors, PlayResult::Draw) => Play::Scissors,
        }
    }
}

impl From<char> for Play {
    fn from(pchar: char) -> Self {
        match pchar {
            'A' | 'X' => Play::Rock,
            'B' | 'Y' => Play::Paper,
            'C' | 'Z' => Play::Scissors,
            _ => panic!("Unknown play {}", pchar),
        }
    }
}

enum PlayResult {
    Win,
    Lose,
    Draw,
}

impl From<char> for PlayResult {
    fn from(pchar: char) -> Self {
        match pchar {
            'A' | 'X' => PlayResult::Lose,
            'B' | 'Y' => PlayResult::Draw,
            'C' | 'Z' => PlayResult::Win,
            _ => panic!("Unknown play {}", pchar),
        }
    }
}

impl PlayResult {
    fn from_play(a: &Play, b: &Play) -> Self {
        match a {
            Play::Rock => match b {
                Play::Rock => PlayResult::Draw,
                Play::Paper => PlayResult::Win,
                Play::Scissors => PlayResult::Lose,
            },
            Play::Paper => match b {
                Play::Rock => PlayResult::Lose,
                Play::Paper => PlayResult::Draw,
                Play::Scissors => PlayResult::Win,
            },
            Play::Scissors => match b {
                Play::Rock => PlayResult::Win,
                Play::Paper => PlayResult::Lose,
                Play::Scissors => PlayResult::Draw,
            },
        }
    }

    fn score(&self) -> u64 {
        match self {
            PlayResult::Win => 6,
            PlayResult::Lose => 0,
            PlayResult::Draw => 3,
        }
    }
}

// Input parsing

type InputEnt = (Play, Play, PlayResult);

fn get_input(day: usize) -> Result<Vec<InputEnt>, Box<dyn Error>> {
    let input = Input::new(day)?;

    parse_input(input.lines())
}

fn parse_input(lines: Lines<BufReader<&[u8]>>) -> Result<Vec<InputEnt>, Box<dyn Error>> {
    let mut result = Vec::new();

    for l in lines {
        let line = l?;

        let chars = line.chars().collect::<Vec<char>>();

        result.push((chars[0].into(), chars[2].into(), chars[2].into()));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use aoc::test_input;

    use super::*;

    const EXAMPLE1: &str = "A Y\nB X\nC Z\n";

    #[test]
    fn test1() {
        let input = parse_input(test_input(EXAMPLE1)).unwrap();
        assert_eq!(part1(&input), 15);
        assert_eq!(part2(&input), 12);
    }
}
