use std::{collections::HashSet, error::Error};

use aoc::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(9, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[Instruction]) -> usize {
    let mut hpos = Pos::default();
    let mut tpos = Pos::default();

    let mut tpositions = HashSet::new();

    tpositions.insert(tpos.clone());

    for i in input {
        let (amt, delta) = i.move_delta();

        for _ in 0..amt {
            hpos.apply(&delta);
            tpos.move_toward(&hpos);
            tpositions.insert(tpos.clone());
        }
    }

    tpositions.len()
}

const TAIL: usize = 9;

fn part2(input: &[Instruction]) -> usize {
    let mut pos: Vec<Pos> = vec![Pos::default(); TAIL + 1];

    let mut tailpositions = HashSet::new();

    tailpositions.insert(pos[TAIL]);

    for i in input {
        let (amt, delta) = i.move_delta();

        for _ in 0..amt {
            let mut new_pos = Vec::with_capacity(TAIL);

            for j in 0..=TAIL {
                let mut elem = pos[j];

                if j == 0 {
                    elem.apply(&delta);
                } else {
                    elem.move_toward(&new_pos[j - 1]);
                }

                new_pos.push(elem);
            }

            pos = new_pos;

            tailpositions.insert(pos[TAIL]);
        }
    }

    tailpositions.len()
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn apply(&mut self, delta: &Delta) {
        self.x += delta.x_add;
        self.y += delta.y_add;
    }

    fn move_toward(&mut self, other: &Pos) {
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;
        let ax_diff = x_diff.abs();
        let ay_diff = y_diff.abs();

        let mut diag = || {
            self.x += x_diff.signum();
            self.y += y_diff.signum();
        };

        if ax_diff == 2 {
            if ay_diff != 0 {
                // Diagonal move
                diag()
            } else {
                // Straight move
                self.x += x_diff.signum();
            }
        } else if ay_diff == 2 {
            if ax_diff != 0 {
                // Diagonal move
                diag()
            } else {
                // Straight move
                self.y += y_diff.signum();
            }
        }
    }
}

struct Delta {
    x_add: isize,
    y_add: isize,
}

impl Delta {
    fn new(x_add: isize, y_add: isize) -> Self {
        Self { x_add, y_add }
    }
}

// Input parsing

enum Instruction {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl Instruction {
    fn move_delta(&self) -> (u32, Delta) {
        use Instruction::*;

        match self {
            Up(amt) => (*amt, Delta::new(0, -1)),
            Down(amt) => (*amt, Delta::new(0, 1)),
            Left(amt) => (*amt, Delta::new(-1, 0)),
            Right(amt) => (*amt, Delta::new(1, 0)),
        }
    }
}

fn input_transform(line: String) -> Instruction {
    let split: Vec<&str> = line.split_whitespace().collect();

    let amt = split[1].parse::<u32>().unwrap();

    match split[0] {
        "U" => Instruction::Up(amt),
        "D" => Instruction::Down(amt),
        "L" => Instruction::Left(amt),
        "R" => Instruction::Right(amt),
        _ => panic!("Unknown instruction {line}"),
    }
}

#[cfg(test)]
mod tests {
    use aoc::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    const EXAMPLE2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test2() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part2(&input), 1);

        let input = parse_test_vec(EXAMPLE2, input_transform).unwrap();
        assert_eq!(part2(&input), 36);
    }
}
