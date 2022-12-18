use std::{cmp::Ordering, collections::HashMap, error::Error};

use floydwarshall::FloydWarshall;
use itertools::Itertools;
use lazy_static::lazy_static;

use aoc::input::parse_input_vec;
use regex::Regex;

use crate::xref::XRef;

mod floydwarshall;
mod xref;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(16, input_transform)?;

    // Build xref
    let xref = XRef::new(&input);

    // Build map
    let dist_map = FloydWarshall::new(&input, &xref);

    // Run parts
    println!("Part 1: {}", part1(&input, &xref, &dist_map));
    println!("Part 2: {}", part2(&input, &xref, &dist_map));

    Ok(())
}

pub fn part1(input: &[InputEnt], xref: &XRef, dist_map: &FloydWarshall) -> usize {
    // Build list of interesting valves
    let valves = input
        .iter()
        .filter_map(|v| {
            if v.rate != 0 {
                Some(xref.index_for_valve(&v.valve))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut best = None;

    walk(
        input,
        dist_map,
        State {
            vno: xref.index_for_valve("AA"),
            valves,
            rate: 0,
            released: 0,
            time_left: 30,
            route: Vec::new(),
        },
        &mut best,
    );

    let (released, _) = best.unwrap();

    released
}

pub fn part2(input: &[InputEnt], xref: &XRef, dist_map: &FloydWarshall) -> usize {
    // Build list of interesting valves
    let valves = input
        .iter()
        .filter_map(|v| {
            if v.rate != 0 {
                Some(xref.index_for_valve(&v.valve))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let sol_map = (0..valves.len())
        .map(|cnt| {
            println!("{}", cnt);
            valves
                .iter()
                .combinations(cnt)
                .map(|valves| {
                    println!("{:?}", valves);
                    let mut best = None;

                    let valves = valves.into_iter().copied().collect::<Vec<_>>();

                    walk(
                        input,
                        dist_map,
                        State {
                            vno: xref.index_for_valve("AA"),
                            valves: valves.clone(),
                            rate: 0,
                            released: 0,
                            time_left: 30,
                            route: Vec::new(),
                        },
                        &mut best,
                    );

                    (valves, best)
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", sol_map);
    0 // TODO
}

fn walk(
    input: &[InputEnt],
    dist_map: &FloydWarshall,
    state: State,
    best: &mut Option<(usize, Vec<u8>)>,
) {
    if state.valves.is_empty() {
        solution(&state, best);
    } else {
        let mut choices = state
            .valves
            .iter()
            .enumerate()
            .map(|(vno, v)| {
                (
                    dist_map.dist_idx(state.vno, *v),
                    input[*v as usize].rate,
                    vno,
                )
            })
            .collect::<Vec<_>>();

        // Sort by distance ascending then rate descending
        choices.sort_by(|a, b| match a.0.cmp(&b.0) {
            Ordering::Equal => b.1.cmp(&a.1),
            c => c,
        });

        for (dist, rate, velem) in choices {
            // TODO other shortcuts here
            if state.time_left > dist {
                let next_vno = state.valves[velem];

                let mut next_valves = state.valves.clone();
                next_valves.swap_remove(velem);

                let time_spent = dist as usize + 1;

                let mut route = state.route.clone();
                route.push(next_vno);

                let next_state = State {
                    vno: next_vno,
                    valves: next_valves,
                    rate: state.rate + rate as usize,
                    released: state.released + (state.rate * time_spent),
                    time_left: state.time_left - time_spent as u8,
                    route,
                };

                walk(input, dist_map, next_state, best);
            } else {
                solution(&state, best);
            }
        }
    }
}

fn solution(state: &State, best: &mut Option<(usize, Vec<u8>)>) {
    let total = state.released + (state.time_left as usize * state.rate);

    match *best {
        Some((best_total, _)) if best_total < total => *best = Some((total, state.route.clone())),
        None => *best = Some((total, state.route.clone())),
        _ => (),
    }
}

struct State {
    vno: u8,
    valves: Vec<u8>,
    rate: usize,
    released: usize,
    time_left: u8,
    route: Vec<u8>,
}

// impl State {
//     fn potential_best(&self) -> usize {
//         let adj = usize::from(self.location != self.valves[0].1);

//         let potential: usize = self
//             .valves
//             .iter()
//             .enumerate()
//             .map(|(i, v)| (self.time_left - min(self.time_left, (i + adj) * 2)) * v.0 as usize)
//             .sum();

//         self.released + (self.time_left * self.rate) + potential
//     }
// }

// Input parsing

#[derive(Clone)]
pub struct InputEnt {
    pub valve: String,
    pub rate: u8,
    pub tunnels: Vec<String>,
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
        let xref = XRef::new(&input);
        let dist_map = FloydWarshall::new(&input, &xref);

        assert_eq!(part1(&input, &xref, &dist_map), 1651);
        assert_eq!(part2(&input, &xref, &dist_map), 1707);
    }
}
