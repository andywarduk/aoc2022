use std::error::Error;

use lazy_static::lazy_static;
use regex::Regex;

use aoc::input::parse_input_vec;
use simulate::{simulate, MineralQty, SimParms};

mod simulate;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(19, input_transform)?;

    // Run parts
    println!("Part 1:");
    part1(&input);

    println!("Part 2:");
    part2(&input);

    Ok(())
}

fn part1(input: &[InputEnt]) {
    let result: usize = input
        .iter()
        .enumerate()
        .map(|(i, blueprint)| {
            let static_state = SimParms::new(blueprint, 24);
            let res = simulate(&static_state);
            println!("  {} => {} : {}", i + 1, res.best, res.builds());
            res.best as usize * (i + 1)
        })
        .sum();

    println!("  Result: {}", result);
}

fn part2(input: &[InputEnt]) {
    let result: usize = input
        .iter()
        .take(3)
        .enumerate()
        .map(|(i, blueprint)| {
            let static_state = SimParms::new(blueprint, 32);
            let res = simulate(&static_state);
            println!("  {} => {} : {}", i + 1, res.best, res.builds());
            res.best as usize
        })
        .product();

    println!("  Result: {}", result);
}

// Input parsing

#[derive(Debug)]
pub struct InputEnt {
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

        let static_state = SimParms::new(&input[0], 24);
        let res = simulate(&static_state);
        println!("{:?}", res.builds);
        assert_eq!(res.best, 9);

        let static_state = SimParms::new(&input[1], 24);
        let res = simulate(&static_state);
        println!("{:?}", res.builds);
        assert_eq!(res.best, 12);
    }

    #[test]
    fn test2() {
        let example1 = EXAMPLE1.replace("\nEach", " Each");
        let input = parse_test_vec(&example1, input_transform).unwrap();

        let static_state = SimParms::new(&input[0], 32);
        let res = simulate(&static_state);
        println!("{:?}", res.builds);
        assert_eq!(res.best, 56);

        let static_state = SimParms::new(&input[1], 32);
        let res = simulate(&static_state);
        println!("{:?}", res.builds);
        assert_eq!(res.best, 62);
    }
}
