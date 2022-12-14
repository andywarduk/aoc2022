use std::error::Error;

use floydwarshall::FloydWarshall;
use lazy_static::lazy_static;

use aoc::input::parse_input_vec;
use regex::Regex;
use walk::{walk, WalkState};

use crate::route::Route;
use crate::xref::XRef;

mod floydwarshall;
mod route;
mod walk;
mod xref;

pub const START_ROOM: &str = "AA";

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(16, input_transform)?;

    // Build xref
    let xref = XRef::new(&input);

    // Build map
    let dist_map = FloydWarshall::new(&input, &xref);

    // Build list of interesting valves
    let valves = interesting_valves(&input, &xref);

    // Run part 1
    println!("Part 1:");
    let (best1, route1) = part1(&input, &xref, &dist_map, &valves);
    println!("  Best: {}", best1);
    print!("  Route: ");
    route1.print(&xref, &dist_map);

    // Run part 2
    println!("Part 2:");
    let (best2, route2_1, route2_2) = part2(&input, &xref, &dist_map, &valves);
    println!("  Best: {}", best2);
    print!("  Route 1: ");
    route2_1.print(&xref, &dist_map);
    print!("  Route 2: ");
    route2_2.print(&xref, &dist_map);

    Ok(())
}

pub fn part1(
    input: &[InputEnt],
    xref: &XRef,
    dist_map: &FloydWarshall,
    valves: &[u8],
) -> (usize, Route) {
    // Best solution found
    let mut best = None;

    // Callback function to maintain best route
    let mut solution = |state: &WalkState| {
        let total = state.released + (state.time_left as usize * state.rate);

        if match best {
            Some(Sol1 { released: best, .. }) if best < total => true,
            None => true,
            _ => false,
        } {
            best = Some(Sol1 {
                released: total,
                route: state.route.clone(),
            })
        }
    };

    // Initial state
    let state = WalkState::new(xref.index_for_valve(START_ROOM), valves.to_vec(), 30);

    // Walk the routes
    walk(input, dist_map, state, &mut solution);

    let best = best.expect("No solutions found");

    (best.released, best.route)
}

pub fn part2(
    input: &[InputEnt],
    xref: &XRef,
    dist_map: &FloydWarshall,
    valves: &[u8],
) -> (usize, Route, Route) {
    // Solutions vectors
    let mut solutions = Vec::new();
    let mut sol_valve_mask = Vec::new(); // Separate to improved cache-locality in the nested loop

    // Callback function to add a solution to the solutions vector
    let mut add_solution = |state: &WalkState| {
        let total = state.released + (state.time_left as usize * state.rate);

        // Add route bitmask
        sol_valve_mask.push(state.route.mask());

        // Add solution
        solutions.push(Sol2 {
            route: state.route.clone(),
            released: total,
        });
    };

    // Initial state
    let state = WalkState::new(xref.index_for_valve(START_ROOM), valves.to_vec(), 26);

    // Walk the routes
    walk(input, dist_map, state, &mut add_solution);

    // Best combination details
    let mut best = 0;
    let mut me_route = None;
    let mut ele_route = None;

    // Walk solutions for me
    for me in 0..solutions.len() {
        let me_mask = sol_valve_mask[me];

        // Walk elephant solutions (from me + 1) which cover a separate set of rooms
        for ele in (me + 1)..solutions.len() {
            let ele_mask = sol_valve_mask[ele];

            if me_mask & ele_mask == 0 {
                // Calculate total
                let total = solutions[me].released + solutions[ele].released;

                // Check against the best
                if total > best {
                    // Best so far
                    best = total;
                    me_route = Some(&solutions[me].route);
                    ele_route = Some(&solutions[ele].route);
                }
            }
        }
    }

    (
        best,
        me_route.expect("No route 1").clone(),
        ele_route.expect("No route 2").clone(),
    )
}

fn interesting_valves(input: &[InputEnt], xref: &XRef) -> Vec<u8> {
    input
        .iter()
        .filter_map(|v| {
            if v.rate != 0 {
                Some(xref.index_for_valve(&v.valve))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

struct Sol1 {
    released: usize,
    route: Route,
}

struct Sol2 {
    released: usize,
    route: Route,
}

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
        let valves = interesting_valves(&input, &xref);

        let (best1, route1) = part1(&input, &xref, &dist_map, &valves);
        let pretty1 = route1.pretty(&xref, &dist_map);
        assert_eq!(best1, 1651);
        assert_eq!(pretty1, "AA DD (open) AA BB (open) AA II JJ (open) II AA DD EE FF GG HH (open) GG FF EE (open) DD CC (open)");

        let (best2, route2_1, route2_2) = part2(&input, &xref, &dist_map, &valves);
        let pretty2_1 = route2_1.pretty(&xref, &dist_map);
        let pretty2_2 = route2_2.pretty(&xref, &dist_map);
        assert_eq!(best2, 1707);
        assert_eq!(pretty2_1, "AA DD (open) EE FF GG HH (open) GG FF EE (open)");
        assert_eq!(pretty2_2, "AA II JJ (open) II AA BB (open) CC (open)");
    }
}
