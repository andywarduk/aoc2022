use std::{cmp::min, collections::HashMap, error::Error};

use lazy_static::lazy_static;

use aoc::input::parse_input_vec;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(16, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

struct MapEnt {
    rate: u8,
    tunnels: Vec<u8>,
}

fn part1(input: &[InputEnt]) -> usize {
    let ivmap: HashMap<u8, String> = input
        .iter()
        .enumerate()
        .map(|(i, v)| (i as u8, v.valve.clone()))
        .collect();

    let vimap: HashMap<String, u8> = input
        .iter()
        .enumerate()
        .map(|(i, v)| (v.valve.clone(), i as u8))
        .collect();

    let map: HashMap<_, _> = input
        .iter()
        .enumerate()
        .map(|(i, v)| {
            (
                i as u8,
                MapEnt {
                    rate: v.rate,
                    tunnels: v.tunnels.iter().map(|t| *vimap.get(t).unwrap()).collect(),
                },
            )
        })
        .collect();

    let mut valves = input
        .iter()
        .enumerate()
        .filter(|(_, v)| v.rate != 0)
        .map(|(i, v)| (v.rate, i as u8))
        .collect::<Vec<_>>();

    valves.sort_by(|a, b| b.cmp(a));

    let state = State {
        location: 0,
        rate: 0,
        valves,
        time_left: 30 + 1,
        released: 0,
        actions: Vec::new(),
    };

    let mut solutions = Solutions { best: None };

    part1_move(&map, *vimap.get("AA").unwrap(), state, &mut solutions);

    println!("{:?}", solutions);

    solutions.best.unwrap().released
}

#[derive(Debug, Clone)]
enum Action {
    Open(u8),
    Move(u8),
}

#[derive(Debug, Clone)]
struct State {
    location: u8,
    rate: usize,
    valves: Vec<(u8, u8)>,
    time_left: usize,
    released: usize,
    actions: Vec<Action>,
}

impl State {
    fn potential_best(&self) -> usize {
        let adj = usize::from(self.location != self.valves[0].1);

        let potential: usize = self
            .valves
            .iter()
            .enumerate()
            .map(|(i, v)| (self.time_left - min(self.time_left, (i + adj) * 2)) * v.0 as usize)
            .sum();

        self.released + (self.time_left * self.rate) + potential
    }

    fn open(&mut self, map_loc: &MapEnt) {
        self.valves.retain(|v| v.1 != self.location);

        self.actions.push(Action::Open(self.location));

        self.decrease_time();

        self.rate += map_loc.rate as usize;
    }

    fn move_to(&mut self, tunnel: u8) {
        self.location = tunnel;

        self.actions.push(Action::Move(tunnel));

        self.decrease_time();
    }

    fn decrease_time(&mut self) {
        self.released += self.rate;
        self.time_left -= 1;
    }
}

#[derive(Debug)]
struct Solutions {
    best: Option<Solution>,
}

impl Solutions {
    fn best_released(&self) -> usize {
        match &self.best {
            None => 0,
            Some(sol) => sol.released,
        }
    }
}

#[derive(Debug)]
struct Solution {
    released: usize,
    action: Vec<Action>,
}

fn part1_iter(map: &HashMap<u8, MapEnt>, state: State, solutions: &mut Solutions) {
    if state.time_left == 0 {
        part1_solution(state, solutions);
    } else if state.valves.is_empty() {
        // No more valves to open
        part1_wait(state, solutions);
    } else if state.potential_best() <= solutions.best_released() {
        // println!(
        //     "Bailing {} {}",
        //     state.potential_best(),
        //     solutions.best_released()
        // );
    } else {
        let map_loc = map.get(&state.location).expect("Valve not found");

        // Open the valve?
        if map_loc.rate > 0 && state.valves.iter().any(|v| v.1 == state.location) {
            part1_open(map, map_loc, state.clone(), solutions)
        }

        // Rank the next moves
        let mut moves = map_loc
            .tunnels
            .iter()
            .map(|t| {
                (
                    match state.valves.iter().position(|(_, v)| v == t) {
                        Some(pos) => pos,
                        None => {
                            match state.actions.iter().position(|a| match a {
                                Action::Move(mv) => t == mv,
                                Action::Open(_) => false,
                            }) {
                                Some(pos) => state.valves.len() + pos + 1,
                                None => state.valves.len(),
                            }
                        }
                    },
                    t,
                )
            })
            .collect::<Vec<_>>();

        moves.sort();

        // Execute the moves
        for (_, t) in moves {
            part1_move(map, *t, state.clone(), solutions);
        }
    }
}

fn part1_open(
    map: &HashMap<u8, MapEnt>,
    map_loc: &MapEnt,
    mut state: State,
    solutions: &mut Solutions,
) {
    // println!("Open {}", state.location);
    state.open(map_loc);
    part1_iter(map, state, solutions);
}

fn part1_move(map: &HashMap<u8, MapEnt>, tunnel: u8, mut state: State, solutions: &mut Solutions) {
    // println!("Move to {}", tunnel);
    state.move_to(tunnel);
    part1_iter(map, state, solutions);
}

fn part1_wait(mut state: State, solutions: &mut Solutions) {
    // println!("Wait at {}", state.location);
    while state.time_left > 0 {
        state.decrease_time()
    }

    part1_solution(state, solutions);
}

fn part1_solution(state: State, solutions: &mut Solutions) {
    println!("Sol {}", state.released);
    if state.released > solutions.best_released() {
        solutions.best = Some(Solution {
            released: state.released,
            action: state.actions,
        })
    }
}

fn part2(input: &[InputEnt]) -> u64 {
    0 // TODO
}

// Input parsing

struct InputEnt {
    valve: String,
    rate: u8,
    tunnels: Vec<String>,
}

fn input_transform(line: String) -> InputEnt {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^Valve (.*?) has flow rate=(\d*); tunnel[s]? lead[s]? to valve[s]? (.*)")
                .unwrap();
    }

    let terms: Vec<&str> = RE
        .captures(&line)
        .unwrap_or_else(|| panic!("Invalid input line: {line}"))
        .iter()
        .skip(1)
        .map(|m| m.expect("Invalid input line").as_str())
        .collect();

    InputEnt {
        valve: terms[0].into(),
        rate: terms[1].parse::<u8>().expect("Invalid flow rate"),
        tunnels: terms[2]
            .split(',')
            .map(|s| String::from(s.trim()))
            .collect::<Vec<_>>(),
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 1651);
        assert_eq!(part2(&input), 0 /* TODO */);
    }
}
