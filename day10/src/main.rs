use std::error::Error;

use aoc::parse_input_vec;
use cpu::Cpu;

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
            line.into_iter()
                .map(|c| if c { '█' } else { ' ' })
                .collect::<String>()
        );
    }

    Ok(())
}

fn part1(input: &[Instruction]) -> isize {
    let mut cpu = Cpu::new(input);
    let mut strength: isize = 0;

    while cpu.tick() {
        if (cpu.cycles() + 20) % 40 == 0 {
            strength += cpu.cycles() as isize * cpu.x_reg()
        }
    }

    strength
}

const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;

fn part2(input: &[Instruction]) -> Vec<Vec<bool>> {
    let mut output = Vec::with_capacity(SCREEN_HEIGHT);
    let mut cur_line = Vec::with_capacity(SCREEN_WIDTH);
    let mut cpu = Cpu::new(input);

    while cpu.tick() {
        let sprite_pos = cpu.x_reg();
        let x = cur_line.len() as isize;

        cur_line.push(x >= sprite_pos - 1 && x <= sprite_pos + 1);

        if x == SCREEN_WIDTH as isize - 1 {
            output.push(cur_line);
            cur_line = Vec::with_capacity(SCREEN_WIDTH);
        }
    }

    if !cur_line.is_empty() {
        output.push(cur_line);
    }

    output
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
        assert_eq!(
            part2(&input)
                .into_iter()
                .map(|l| l
                    .into_iter()
                    .map(|p| if p { '#' } else { '.' })
                    .collect::<String>())
                .collect::<Vec<_>>(),
            EXAMPLE_RESULT
        );
    }
}
