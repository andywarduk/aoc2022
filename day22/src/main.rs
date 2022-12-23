use std::error::Error;

use aoc::input::parse_input_vec;

mod map;
mod part1;
mod part2;

use part1::part1;
use part2::part2;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let mut input = parse_input_vec(22, input_transform)?;
    let instructions = split_instructions(&mut input);
    let edge_map = [9, 8, 5, 4, 3, 2, 7, 6, 1, 0, 13, 12, 11, 10];

    // Run parts
    println!("Part 1: {}", part1(&input, &instructions, 50, &edge_map));
    println!("Part 2: {}", part2(&input, &instructions, 50, &edge_map));

    Ok(())
}

#[derive(Debug)]
pub enum Instruction {
    Forward(u8),
    Left,
    Right,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    #[default]
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

// Input parsing

fn split_instructions(input: &mut Vec<String>) -> Vec<Instruction> {
    let line = input.pop().unwrap();
    let mut instructions = Vec::new();

    let mut num_start = None;

    let flushnum =
        |num_start: &mut Option<usize>, end: usize, instructions: &mut Vec<Instruction>| {
            if let Some(start) = num_start {
                instructions.push(Instruction::Forward(
                    line[*start..end].parse::<u8>().unwrap(),
                ));

                *num_start = None;
            }
        };

    line.chars().enumerate().for_each(|(i, c)| match c {
        'L' => {
            flushnum(&mut num_start, i, &mut instructions);
            instructions.push(Instruction::Left);
        }
        'R' => {
            flushnum(&mut num_start, i, &mut instructions);
            instructions.push(Instruction::Right);
        }
        '0'..='9' => {
            if num_start.is_none() {
                num_start = Some(i);
            }
        }
        _ => panic!("Invalid character"),
    });

    flushnum(&mut num_start, line.len(), &mut instructions);

    input.retain(|i| !i.is_empty());

    instructions
}

fn input_transform(line: String) -> String {
    line
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

    //  a',b',c',c,b,d',e',f',f,e, d, a,g', g
    // [ 0, 1, 2,3,4, 5, 6, 7,8,9,10,11,12,13]
    // [11, 4, 3,2,1,10, 9, 8,7,6, 5, 0,13,12]

    #[test]
    fn test1() {
        let mut input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let instructions = split_instructions(&mut input);
        let edge_map = [11, 4, 3, 2, 1, 10, 9, 8, 7, 6, 5, 0, 13, 12];

        assert_eq!(part1(&input, &instructions, 4, &edge_map), 6032);
        assert_eq!(part2(&input, &instructions, 4, &edge_map), 5031);
    }
}
