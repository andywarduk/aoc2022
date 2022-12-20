use std::cmp::max;

use crate::InputEnt;

pub type MineralQty = u8;
pub type RobotQty = u8;
pub type TimeQty = u8;

/// What to build next / what got built
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Build {
    #[default]
    Any,
    Ore,
    Clay,
    Obsidian,
    Geode,
}

/// Simulation parameters
pub struct SimParms<'a> {
    blueprint: &'a InputEnt,
    time: TimeQty,
    max_ore: MineralQty,
}

impl<'a> SimParms<'a> {
    /// Create new simulation parameters
    pub fn new(blueprint: &'a InputEnt, time: TimeQty) -> Self {
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

/// Simulation result
#[derive(Default)]
pub struct SimResult {
    pub best: MineralQty,
    pub builds: Vec<(TimeQty, Build)>,
}

impl SimResult {
    pub fn builds(&self) -> String {
        let elems = self
            .builds
            .iter()
            .map(|(time, built)| format!("{}:{:?}", time, built))
            .collect::<Vec<_>>();

        if elems.is_empty() {
            "None".to_string()
        } else {
            elems.join(" ")
        }
    }
}
/// Run simulation
pub fn simulate(static_state: &SimParms) -> SimResult {
    let mut result = SimResult::default();

    let state = State {
        ore_robots: 1,
        ..State::default()
    };

    simulate_iter(state, static_state, &mut result);

    result
}

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

fn simulate_iter(old_state: State, sim_parms: &SimParms, result: &mut SimResult) {
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
    if upd_state.time_used == sim_parms.time {
        // Best result yet?
        if upd_state.geodes > result.best {
            // Yes - update result
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
        let (built, next_state) = if old_state.ore >= sim_parms.blueprint.geode_robot_ore
            && old_state.obsidian >= sim_parms.blueprint.geode_robot_obsidian
        {
            // Build the robot
            let mut new_builds = old_state.builds.clone();
            new_builds.push((old_state.time_used, Build::Geode));

            (
                true,
                State {
                    ore: upd_state.ore - sim_parms.blueprint.geode_robot_ore,
                    obsidian: upd_state.obsidian - sim_parms.blueprint.geode_robot_obsidian,
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

        simulate_iter(next_state, sim_parms, result);

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
                        < sim_parms.blueprint.geode_robot_obsidian =>
            {
                true
            }
            Build::Obsidian => true,
            _ => false,
        } {
            let next_state = if old_state.ore >= sim_parms.blueprint.obsidian_robot_ore
                && old_state.clay >= sim_parms.blueprint.obsidian_robot_clay
            {
                // Build the robot
                let mut new_builds = old_state.builds.clone();
                new_builds.push((old_state.time_used, Build::Obsidian));

                State {
                    ore: upd_state.ore - sim_parms.blueprint.obsidian_robot_ore,
                    clay: upd_state.clay - sim_parms.blueprint.obsidian_robot_clay,
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

            simulate_iter(next_state, sim_parms, result);
        }

        // Clay robot
        if match old_state.next_build {
            Build::Any
                if (old_state.clay_robots as MineralQty)
                    < sim_parms.blueprint.obsidian_robot_clay =>
            {
                true
            }
            Build::Clay => true,
            _ => false,
        } {
            let next_state = if old_state.ore >= sim_parms.blueprint.clay_robot_ore {
                // Build the robot
                let mut new_builds = old_state.builds.clone();
                new_builds.push((old_state.time_used, Build::Clay));

                State {
                    next_build: Build::Any,
                    ore: upd_state.ore - sim_parms.blueprint.clay_robot_ore,
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

            simulate_iter(next_state, sim_parms, result);
        }

        // Ore robot
        if match old_state.next_build {
            Build::Any if (old_state.ore_robots as MineralQty) < sim_parms.max_ore => true,
            Build::Ore => true,
            _ => false,
        } {
            let next_state = if old_state.ore >= sim_parms.blueprint.ore_robot_ore {
                // Build the robot
                let mut new_builds = old_state.builds.clone();
                new_builds.push((old_state.time_used, Build::Ore));

                State {
                    next_build: Build::Any,
                    ore: upd_state.ore - sim_parms.blueprint.ore_robot_ore,
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

            simulate_iter(next_state, sim_parms, result);
        }
    }
}
