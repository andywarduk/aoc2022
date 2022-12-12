use std::{
    collections::{HashSet, VecDeque},
    error::Error,
};

use aoc::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(12, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> usize {
    let width = input[0].len() as u16;
    let height = input.len() as u16;

    let mut stack = VecDeque::new();
    stack.push_back(Visited::new(find_pos(input, START), 0));

    let end = find_pos(input, END);

    let mut visited = HashSet::new();

    'found: loop {
        let cur_pos = stack.pop_front().expect("No positions in the stack");

        visited.insert(cur_pos.pos.clone());

        let neighbours =
            cur_pos
                .pos
                .neighbours(input, width, height, |to, from| from <= to + 1, &visited);

        for n in neighbours {
            if n == end {
                break 'found cur_pos.dist + 1;
            }

            visited.insert(n.clone());
            stack.push_back(Visited::new(n, cur_pos.dist + 1));
        }
    }
}

fn part2(input: &[InputEnt]) -> usize {
    let width = input[0].len() as u16;
    let height = input.len() as u16;

    let mut stack = VecDeque::new();
    stack.push_back(Visited::new(find_pos(input, END), 0));

    let mut visited = HashSet::new();

    'found: loop {
        let cur_pos = stack.pop_front().expect("No positions in the stack");

        visited.insert(cur_pos.pos.clone());

        let neighbours =
            cur_pos
                .pos
                .neighbours(input, width, height, |to, from| to <= from + 1, &visited);

        for n in neighbours {
            if n.height(input) == 0 {
                break 'found cur_pos.dist + 1;
            }

            visited.insert(n.clone());
            stack.push_back(Visited::new(n, cur_pos.dist + 1));
        }
    }
}

fn find_pos(input: &[InputEnt], find_id: i8) -> Pos {
    input
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, id)| {
                if *id == find_id {
                    Some(Pos::new(x as u16, y as u16))
                } else {
                    None
                }
            })
        })
        .expect("Start position not found")
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Pos {
    x: u16,
    y: u16,
}

impl Pos {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    fn height(&self, input: &[InputEnt]) -> i8 {
        match input[self.y as usize][self.x as usize] {
            START => 0,
            END => 26,
            h => h,
        }
    }

    fn neighbours<F>(
        &self,
        input: &[InputEnt],
        width: u16,
        height: u16,
        chk: F,
        visited: &HashSet<Pos>,
    ) -> Vec<Pos>
    where
        F: Fn(i8, i8) -> bool,
    {
        let mut neigh = Vec::new();
        let self_height = self.height(input);

        let mut add = |x, y| {
            let pos = Pos::new(x, y);
            let chk_height = pos.height(input);

            if chk(self_height, chk_height) && !visited.contains(&pos) {
                neigh.push(pos);
            }
        };

        if self.x > 0 {
            add(self.x - 1, self.y);
        }

        if self.x < width - 1 {
            add(self.x + 1, self.y);
        }

        if self.y > 0 {
            add(self.x, self.y - 1);
        }

        if self.y < height - 1 {
            add(self.x, self.y + 1);
        }

        neigh
    }
}

struct Visited {
    pos: Pos,
    dist: usize,
}

impl Visited {
    fn new(pos: Pos, dist: usize) -> Self {
        Self { pos, dist }
    }
}
// Input parsing

const START: i8 = -16;
const END: i8 = -32;

type InputEnt = Vec<i8>;

fn input_transform(line: String) -> InputEnt {
    line.chars()
        .map(|c| match c {
            'S' => START,
            'E' => END,
            'a'..='z' => c as i8 - b'a' as i8,
            _ => panic!("Unepected char"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 31);
        assert_eq!(part2(&input), 29);
    }
}
