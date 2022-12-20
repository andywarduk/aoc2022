use std::{cmp::max, error::Error};

use lazy_static::lazy_static;
use regex::Regex;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(19, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(i, blueprint)| {
            let static_state = StaticState::new(blueprint, 24);
            let res = simulate(&static_state);
            res.best as usize * (i + 1)
        })
        .sum()
}

fn part2(input: &[InputEnt]) -> usize {
    input
        .iter()
        .take(3)
        .map(|blueprint| {
            let static_state = StaticState::new(blueprint, 32);
            let res = simulate(&static_state);
            res.best as usize
        })
        .product()
}

struct StaticState<'a> {
    blueprint: &'a InputEnt,
    time: TimeQty,
    max_ore: MineralQty,
}

impl<'a> StaticState<'a> {
    fn new(blueprint: &'a InputEnt, time: TimeQty) -> Self {
        let max_ore = max(
            max(blueprint.clay_robot_ore, blueprint.obsidian_robot_ore),
            blueprint.geode_robot_ore,
        );

        Self {
            blueprint,
            time,
            max_ore,
        }
    }
}

#[derive(Default)]
struct SimResult {
    best: MineralQty,
    builds: Vec<(TimeQty, Build)>,
}

fn simulate(static_state: &StaticState) -> SimResult {
    let mut result = SimResult::default();

    let state = State {
        ore_robots: 1,
        ..State::default()
    };

    simulate_iter(state, static_state, &mut result);

    result
}

fn simulate_iter(old_state: State, static_state: &StaticState, result: &mut SimResult) {
    // Create new state
    let mut upd_state = old_state.clone();

    upd_state.time_used += 1;

    // -- Collections --

    // Ore robot action
    upd_state.ore += old_state.ore_robots as MineralQty;

    // Clay robot action
    upd_state.clay += old_state.clay_robots as MineralQty;

    // Obsidian robot action
    upd_state.obsidian += old_state.obsidian_robots as MineralQty;

    // Geode robot action
    upd_state.geodes += old_state.geode_robots as MineralQty;

    // Out of time?
    if upd_state.time_used == static_state.time {
        if upd_state.geodes > result.best {
            result.best = upd_state.geodes;
            result.builds = upd_state.builds;
        }

        return;
    }

    // -- Actions --

    // Geode robot
    let geode_built = if match old_state.next_build {
        Build::Any
            if old_state.ore_robots > 0
                && old_state.clay_robots > 0
                && old_state.obsidian_robots > 0 =>
        {
            true
        }
        Build::Geode => true,
        _ => false,
    } {
        let (built, next_state) = if old_state.ore >= static_state.blueprint.geode_robot_ore
            && old_state.obsidian >= static_state.blueprint.geode_robot_obsidian
        {
            // Build the robot
            let mut new_builds = old_state.builds.clone();
            new_builds.push((old_state.time_used, Build::Geode));

            (
                true,
                State {
                    ore: upd_state.ore - static_state.blueprint.geode_robot_ore,
                    obsidian: upd_state.obsidian - static_state.blueprint.geode_robot_obsidian,
                    geode_robots: old_state.geode_robots + 1,
                    next_build: Build::Any,
                    builds: new_builds,
                    ..upd_state
                },
            )
        } else {
            // Wait for materials
            (
                false,
                State {
                    next_build: Build::Geode,
                    ..upd_state
                },
            )
        };

        simulate_iter(next_state, static_state, result);

        built
    } else {
        false
    };

    if !geode_built {
        // Obsidian robot
        if match old_state.next_build {
            Build::Any
                if old_state.ore_robots > 0
                    && old_state.clay_robots > 0
                    && (old_state.obsidian_robots as MineralQty)
                        < static_state.blueprint.geode_robot_obsidian =>
            {
                true
            }
            Build::Obsidian => true,
            _ => false,
        } {
            let next_state = if old_state.ore >= static_state.blueprint.obsidian_robot_ore
                && old_state.clay >= static_state.blueprint.obsidian_robot_clay
            {
                // Build the robot
                let mut new_builds = old_state.builds.clone();
                new_builds.push((old_state.time_used, Build::Obsidian));

                State {
                    ore: upd_state.ore - static_state.blueprint.obsidian_robot_ore,
                    clay: upd_state.clay - static_state.blueprint.obsidian_robot_clay,
                    obsidian_robots: old_state.obsidian_robots + 1,
                    next_build: Build::Any,
                    builds: new_builds,
                    ..upd_state
                }
            } else {
                // Wait for materials
                State {
                    next_build: Build::Obsidian,
                    builds: old_state.builds.clone(),
                    ..upd_state
                }
            };

            simulate_iter(next_state, static_state, result);
        }

        // Clay robot
        if match old_state.next_build {
            Build::Any
                if (old_state.clay_robots as MineralQty)
                    < static_state.blueprint.obsidian_robot_clay =>
            {
                true
            }
            Build::Clay => true,
            _ => false,
        } {
            let next_state = if old_state.ore >= static_state.blueprint.clay_robot_ore {
                // Build the robot
                let mut new_builds = old_state.builds.clone();
                new_builds.push((old_state.time_used, Build::Clay));

                State {
                    next_build: Build::Any,
                    ore: upd_state.ore - static_state.blueprint.clay_robot_ore,
                    clay_robots: old_state.clay_robots + 1,
                    builds: new_builds,
                    ..upd_state
                }
            } else {
                // Wait for materials
                State {
                    next_build: Build::Clay,
                    builds: old_state.builds.clone(),
                    ..upd_state
                }
            };

            simulate_iter(next_state, static_state, result);
        }

        // Ore robot
        if match old_state.next_build {
            Build::Any if (old_state.ore_robots as MineralQty) < static_state.max_ore => true,
            Build::Ore => true,
            _ => false,
        } {
            let next_state = if old_state.ore >= static_state.blueprint.ore_robot_ore {
                // Build the robot
                let mut new_builds = old_state.builds.clone();
                new_builds.push((old_state.time_used, Build::Ore));

                State {
                    next_build: Build::Any,
                    ore: upd_state.ore - static_state.blueprint.ore_robot_ore,
                    ore_robots: old_state.ore_robots + 1,
                    builds: new_builds,
                    ..upd_state
                }
            } else {
                // Wait for materials
                State {
                    next_build: Build::Ore,
                    builds: old_state.builds,
                    ..upd_state
                }
            };

            simulate_iter(next_state, static_state, result);
        }
    }
}

type MineralQty = u8;
type RobotQty = u8;
type TimeQty = u8;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct State {
    time_used: TimeQty,
    ore: MineralQty,
    clay: MineralQty,
    obsidian: MineralQty,
    geodes: MineralQty,
    ore_robots: RobotQty,
    clay_robots: RobotQty,
    obsidian_robots: RobotQty,
    geode_robots: RobotQty,
    next_build: Build,
    builds: Vec<(TimeQty, Build)>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum Build {
    #[default]
    Any,
    Ore,
    Clay,
    Obsidian,
    Geode,
}

// Input parsing

#[derive(Debug)]
struct InputEnt {
    ore_robot_ore: MineralQty,
    clay_robot_ore: MineralQty,
    obsidian_robot_ore: MineralQty,
    obsidian_robot_clay: MineralQty,
    geode_robot_ore: MineralQty,
    geode_robot_obsidian: MineralQty,
}

fn input_transform(line: String) -> InputEnt {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"Blueprint \d*: Each ore robot costs (\d*) ore. Each clay robot costs (\d*) ore. Each obsidian robot costs (\d*) ore and (\d*) clay. Each geode robot costs (\d*) ore and (\d*) obsidian.")
                .unwrap();
    }

    let nums: Vec<MineralQty> = RE
        .captures(&line)
        .unwrap_or_else(|| panic!("Invalid input line: {line}"))
        .iter()
        .skip(1)
        .map(|m| {
            m.expect("Invalid input line")
                .as_str()
                .parse::<MineralQty>()
                .expect("Invalid number")
        })
        .collect();

    InputEnt {
        ore_robot_ore: nums[0],
        clay_robot_ore: nums[1],
        obsidian_robot_ore: nums[2],
        obsidian_robot_clay: nums[3],
        geode_robot_ore: nums[4],
        geode_robot_obsidian: nums[5],
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "Blueprint 1:
Each ore robot costs 4 ore.
Each clay robot costs 2 ore.
Each obsidian robot costs 3 ore and 14 clay.
Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2:
Each ore robot costs 2 ore.
Each clay robot costs 3 ore.
Each obsidian robot costs 3 ore and 8 clay.
Each geode robot costs 3 ore and 12 obsidian.
";

    #[test]
    fn test1() {
        let example1 = EXAMPLE1.replace("\nEach", " Each");
        let input = parse_test_vec(&example1, input_transform).unwrap();

        let static_state = StaticState::new(&input[0], 24);
        let res = simulate(&static_state);
        println!("{:?}", res.builds);
        assert_eq!(res.best, 9);

        let static_state = StaticState::new(&input[1], 24);
        let res = simulate(&static_state);
        println!("{:?}", res.builds);
        assert_eq!(res.best * 2, 12);

        assert_eq!(part1(&input), 33);
    }

    #[test]
    fn test2() {
        let example1 = EXAMPLE1.replace("\nEach", " Each");
        let input = parse_test_vec(&example1, input_transform).unwrap();

        let static_state = StaticState::new(&input[0], 32);
        let res = simulate(&static_state);
        println!("{:?}", res.builds);
        assert_eq!(res.best, 56);

        let static_state = StaticState::new(&input[1], 32);
        let res = simulate(&static_state);
        println!("{:?}", res.builds);
        assert_eq!(res.best, 62);
    }
}
