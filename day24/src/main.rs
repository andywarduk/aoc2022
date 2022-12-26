use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::error::Error;

use aoc::input::parse_input_vec;
use input::InputEnt;
use pos::Pos;

use crate::input::input_transform;
use crate::map::Map;

mod dir;
mod input;
mod map;
mod pos;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(24, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> usize {
    let map = Map::new(input);

    shortest_path(&map, 1, map.entry.clone(), map.exit.clone())
}

fn part2(input: &[InputEnt]) -> usize {
    let map = Map::new(input);

    let time = shortest_path(&map, 1, map.entry.clone(), map.exit.clone());
    let time = shortest_path(&map, time, map.exit.clone(), map.entry.clone());
    shortest_path(&map, time, map.entry.clone(), map.exit.clone())
}

fn shortest_path(map: &Map, start_time: usize, start_pos: Pos, end_pos: Pos) -> usize {
    let mut states = BinaryHeap::new();

    states.push(State {
        time: start_time,
        pos: start_pos.clone(),
        dist: end_pos.dist(&start_pos),
    });

    let mut min_time = None;

    let mut visited = HashSet::new();

    while let Some(state) = states.pop() {
        if let Some(min_time) = min_time {
            if min_time < state.time || state.dist >= min_time - state.time {
                continue;
            }
        }

        // Get blizzard positions
        let blizzards = &map.blizzards[state.time % map.blizzards.len()];

        // Calculate valid next positions
        let mut next_positions = Vec::with_capacity(5);

        let mut add_next = |x, y| {
            let pos = Pos { x, y };
            if ((x > 0 && x <= map.width && y > 0 && y <= map.height)
                || pos == end_pos
                || pos == start_pos)
                && !blizzards.contains(&pos)
                && !visited.contains(&(pos.clone(), state.time + 1))
            {
                next_positions.push(pos);
            }
        };

        add_next(state.pos.x + 1, state.pos.y);
        add_next(state.pos.x, state.pos.y + 1);
        add_next(state.pos.x - 1, state.pos.y);
        if state.pos.y > 0 {
            add_next(state.pos.x, state.pos.y - 1);
        }
        add_next(state.pos.x, state.pos.y);

        for p in next_positions {
            if p == end_pos {
                min_time = Some(state.time);
            } else {
                let mut next_state = state.clone();
                next_state.time = state.time + 1;
                next_state.dist = end_pos.dist(&p);
                next_state.pos = p.clone();
                states.push(next_state);
                visited.insert((p, state.time + 1));
            }
        }
    }

    min_time.unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    time: usize,
    pos: Pos,
    dist: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| other.time.cmp(&self.time))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 18);
        assert_eq!(part2(&input), 54);
    }
}
