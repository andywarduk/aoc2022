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
    move_rope(input, 1)
}

fn part2(input: &[Instruction]) -> usize {
    move_rope(input, 9)
}

fn move_rope(input: &[Instruction], tail_cnt: usize) -> usize {
    let mut pos = vec![Pos::default(); tail_cnt + 1];
    let mut tailpositions = HashSet::new();

    tailpositions.insert(pos[tail_cnt]);

    for (amt, delta) in input {
        for _ in 0..*amt {
            for j in 0..=tail_cnt {
                let mut elem = pos[j];

                if j == 0 {
                    elem.apply_delta(delta);
                } else {
                    elem.move_toward(&pos[j - 1]);
                }

                pos[j] = elem;
            }

            tailpositions.insert(pos[tail_cnt]);
        }
    }

    tailpositions.len()
}

type Delta = (isize, isize);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn apply_delta(&mut self, delta: &Delta) {
        self.x += delta.0;
        self.y += delta.1;
    }

    fn move_toward(&mut self, other: &Pos) {
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;

        let ax_diff = x_diff.abs();
        let ay_diff = y_diff.abs();

        if (ax_diff == 2 && ay_diff != 0) || (ay_diff == 2 && ax_diff != 0) {
            // Diagonal move
            self.x += x_diff.signum();
            self.y += y_diff.signum();
        } else if ax_diff == 2 {
            // Straight move along x
            self.x += x_diff.signum();
        } else if ay_diff == 2 {
            // Straight move along y
            self.y += y_diff.signum();
        }
    }
}

// Input parsing

type Instruction = (u32, Delta);

fn input_transform(line: String) -> Instruction {
    let split: Vec<&str> = line.split_whitespace().collect();

    let amt = split[1].parse::<u32>().unwrap();

    match split[0] {
        "U" => (amt, (0, -1)),
        "D" => (amt, (0, 1)),
        "L" => (amt, (-1, 0)),
        "R" => (amt, (1, 0)),
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
