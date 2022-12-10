use std::error::Error;

use aoc::parse_input_vec;
use cpu::{Cpu, TickAction};

mod cpu;

use crate::cpu::Instruction;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(10, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));

    println!("Part 2:");
    for line in part2(&input) {
        println!(
            "  {}",
            line.chars()
                .into_iter()
                .map(|c| if c == '#' { '█' } else { ' ' })
                .collect::<String>()
        );
    }

    Ok(())
}

fn part1(input: &[Instruction]) -> isize {
    let mut cpu = Cpu::new(input);
    let mut strength: isize = 0;

    loop {
        match cpu.tick() {
            TickAction::Halt => break,
            TickAction::Executing => {
                if (cpu.cycles() + 20) % 40 == 0 {
                    strength += cpu.cycles() as isize * cpu.x_reg()
                }
            }
        }
    }

    strength
}

fn part2(input: &[Instruction]) -> Vec<String> {
    let mut output = vec![vec![' '; 40]; 6];
    let mut x = 0;
    let mut y = 0;
    let mut cpu = Cpu::new(input);

    loop {
        match cpu.tick() {
            TickAction::Halt => break,
            TickAction::Executing => {
                let pos = cpu.x_reg();

                output[y][x] = if x as isize >= pos - 1 && x as isize <= pos + 1 {
                    '#'
                } else {
                    '.'
                };

                x += 1;

                if x == 40 {
                    x = 0;
                    y += 1;
                }
            }
        }
    }

    output
        .into_iter()
        .map(|chars| chars.into_iter().collect::<String>())
        .collect()
}

// Input parsing

fn input_transform(line: String) -> Instruction {
    let mut split = line.split_whitespace();

    match split.next() {
        Some("noop") => Instruction::NoOp,
        Some("addx") => Instruction::AddX(
            split
                .next()
                .expect("addx needs an argument")
                .parse()
                .expect("addx argument not numeric"),
        ),
        _ => panic!("Unknown instruction {line}"),
    }
}

#[cfg(test)]
mod tests {
    use aoc::parse_test_input_vec;

    use super::*;

    const EXAMPLE_RESULT: [&str; 6] = [
        "##..##..##..##..##..##..##..##..##..##..",
        "###...###...###...###...###...###...###.",
        "####....####....####....####....####....",
        "#####.....#####.....#####.....#####.....",
        "######......######......######......####",
        "#######.......#######.......#######.....",
    ];

    #[test]
    fn test1() {
        let input = parse_test_input_vec(10, 1, input_transform).unwrap();
        assert_eq!(part1(&input), 13140);
        assert_eq!(part2(&input), EXAMPLE_RESULT);
    }
}
