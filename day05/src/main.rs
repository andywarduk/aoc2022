use std::{collections::VecDeque, error::Error};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let (stacks, moves) = get_input(parse_input_vec(5, input_transform)?)?;

    // Run parts
    println!("Part 1: {}", part1(stacks.clone(), &moves));
    println!("Part 2: {}", part2(stacks, &moves));

    Ok(())
}

fn part1(mut stacks: Stacks, moves: &[Move]) -> String {
    for mv in moves {
        stacks.move_lifo(mv);
    }

    stacks.top_boxes()
}

fn part2(mut stacks: Stacks, moves: &[Move]) -> String {
    for mv in moves {
        stacks.move_chunk(mv);
    }

    stacks.top_boxes()
}

#[derive(Clone)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn top_boxes(&self) -> String {
        self.stacks.iter().map(|s| s.last().unwrap()).collect()
    }

    fn move_lifo(&mut self, mv: &Move) {
        for _ in 0..mv.count {
            let item = self.stacks[mv.from].pop().unwrap();
            self.stacks[mv.to].push(item);
        }
    }

    fn move_chunk(&mut self, mv: &Move) {
        let pos = self.stacks[mv.from].len() - mv.count;
        let mut items = self.stacks[mv.from].split_off(pos);
        self.stacks[mv.to].append(&mut items);
    }
}

impl From<Vec<VecDeque<char>>> for Stacks {
    fn from(vvdq: Vec<VecDeque<char>>) -> Self {
        Self {
            stacks: vvdq.into_iter().map(Vec::from).collect(),
        }
    }
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

// Input parsing

fn get_input(lines: Vec<InputEnt>) -> Result<(Stacks, Vec<Move>), Box<dyn Error>> {
    let (stacks, moves) = lines.into_iter().fold(
        (Vec::new(), Vec::new()),
        |(mut stacks, mut moves), line| match line {
            InputEnt::StackLine(s) => {
                for (i, c) in s.iter().enumerate() {
                    if !c.is_ascii_whitespace() {
                        while stacks.len() < i + 1 {
                            stacks.push(VecDeque::new())
                        }
                        stacks[i].push_front(*c)
                    }
                }
                (stacks, moves)
            }
            InputEnt::MoveLine(m) => {
                moves.push(m);
                (stacks, moves)
            }
            _ => (stacks, moves),
        },
    );

    Ok((stacks.into(), moves))
}

enum InputEnt {
    StackLine(Vec<char>),
    MoveLine(Move),
    Ignore,
}

fn input_transform(line: String) -> InputEnt {
    if line.trim_start().starts_with('[') {
        InputEnt::StackLine(line.chars().skip(1).step_by(4).collect())
    } else if line.starts_with("move") {
        let split: Vec<&str> = line.split_ascii_whitespace().collect();

        InputEnt::MoveLine(Move {
            count: split[1].parse::<usize>().unwrap(),
            from: split[3].parse::<usize>().unwrap() - 1,
            to: split[5].parse::<usize>().unwrap() - 1,
        })
    } else {
        InputEnt::Ignore
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

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
        let (stacks, moves) =
            get_input(parse_test_vec(EXAMPLE1, input_transform).unwrap()).unwrap();
        assert_eq!(part1(stacks.clone(), &moves), "CMZ");
        assert_eq!(part2(stacks, &moves), "MCD");
    }
}
