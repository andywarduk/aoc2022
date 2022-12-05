use std::{collections::VecDeque, error::Error};

use aoc::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let (stack, moves) = get_input(parse_input_vec(5, input_transform)?)?;

    // Run parts
    println!("Part 1: {}", part1(stack.clone(), &moves));
    println!("Part 2: {}", part2(stack.clone(), &moves));

    Ok(())
}

fn part1(mut stack: Stack, moves: &[Move]) -> String {
    for m in moves {
        for _ in 0..m.count {
            let item = stack.0[m.from].pop().unwrap();
            stack.0[m.to].push(item);
        }
    }

    stack
        .0
        .iter()
        .map(|s| s.last().unwrap())
        .collect::<String>()
}

fn part2(mut stack: Stack, moves: &[Move]) -> String {
    for m in moves {
        let pos = stack.0[m.from].len() - m.count;
        let mut items = stack.0[m.from].split_off(pos);
        stack.0[m.to].append(&mut items);
    }

    stack
        .0
        .iter()
        .map(|s| s.last().unwrap())
        .collect::<String>()
}

#[derive(Clone, Debug)]
struct Stack(Vec<Vec<char>>);

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

// Input parsing

fn get_input(lines: Vec<InputEnt>) -> Result<(Stack, Vec<Move>), Box<dyn Error>> {
    let mut stack = Vec::new();
    let mut moves = Vec::new();

    let elems = (lines.first().unwrap().len() + 3) / 4;

    for _ in 0..elems {
        stack.push(VecDeque::new());
    }

    for line in lines {
        if line.trim_start().starts_with('[') {
            let chars: Vec<char> = line.chars().collect();

            let mut idx = 1;
            for i in 0..elems {
                if chars[idx] != ' ' {
                    stack[i].push_front(chars[idx]);
                }
                idx += 4;
            }
        } else if line.starts_with("move") {
            let split: Vec<&str> = line.split_ascii_whitespace().collect();

            moves.push(Move {
                count: split[1].parse::<usize>()?,
                from: split[3].parse::<usize>()? - 1,
                to: split[5].parse::<usize>()? - 1,
            })
        }
    }

    let stack = stack.into_iter().map(Vec::from).collect();

    Ok((Stack(stack), moves))
}

type InputEnt = String;

fn input_transform(line: String) -> InputEnt {
    line
}

#[cfg(test)]
mod tests {
    use aoc::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test1() {
        let (stack, moves) = get_input(parse_test_vec(EXAMPLE1, input_transform).unwrap()).unwrap();
        assert_eq!(part1(stack.clone(), &moves), "CMZ");
        assert_eq!(part2(stack.clone(), &moves), "MCD");
    }
}
