use std::cmp::max;

use crate::InputEnt;

pub type MineralQty = u16;
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

    let robots = Robots {
        ore: 1,
        ..Default::default()
    };

    let state = State {
        robots,
        ..Default::default()
    };

    simulate_iter(state, static_state, &mut result);

    result
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Inventory {
    ore: MineralQty,
    clay: MineralQty,
    obsidian: MineralQty,
    geodes: MineralQty,
}

impl Inventory {
    fn collect(&mut self, robots: &Robots) {
        // Ore robot action
        self.ore += robots.ore as MineralQty;

        // Clay robot action
        self.clay += robots.clay as MineralQty;

        // Obsidian robot action
        self.obsidian += robots.obsidian as MineralQty;

        // Geode robot action
        self.geodes += robots.geode as MineralQty;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Robots {
    ore: RobotQty,
    clay: RobotQty,
    obsidian: RobotQty,
    geode: RobotQty,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct State {
    time_used: TimeQty,
    inventory: Inventory,
    robots: Robots,
    next_build: Build,
    builds: Vec<(TimeQty, Build)>,
}

fn simulate_iter(old_state: State, sim_parms: &SimParms, result: &mut SimResult) {
    // Update time used
    let new_time_used = old_state.time_used + 1;

    // -- Collections --

    // Create new inventory
    let mut upd_inventory = old_state.inventory.clone();

    upd_inventory.collect(&old_state.robots);

    // Out of time?
    if new_time_used == sim_parms.time {
        // Best result yet?
        if upd_inventory.geodes > result.best {
            // Yes - update result
            result.best = upd_inventory.geodes;
            result.builds = old_state.builds;
        }

        return;
    }

    // -- Actions --

    // Geode robot
    let geode_built = if match old_state.next_build {
        Build::Geode => true,
        Build::Any
            if old_state.robots.ore > 0
                && old_state.robots.clay > 0
                && old_state.robots.obsidian > 0 =>
        {
            true
        }
        _ => false,
    } {
        let (built, next_state) = if old_state.inventory.ore >= sim_parms.blueprint.geode_robot_ore
            && old_state.inventory.obsidian >= sim_parms.blueprint.geode_robot_obsidian
        {
            // Build the robot
            let mut new_builds = old_state.builds.clone();
            new_builds.push((old_state.time_used, Build::Geode));

            let mut new_inventory = upd_inventory.clone();
            new_inventory.ore -= sim_parms.blueprint.geode_robot_ore;
            new_inventory.obsidian -= sim_parms.blueprint.geode_robot_obsidian;

            let mut new_robots = old_state.robots.clone();
            new_robots.geode += 1;

            (
                true,
                State {
                    time_used: new_time_used,
                    inventory: new_inventory,
                    robots: new_robots,
                    next_build: Build::Any,
                    builds: new_builds,
                },
            )
        } else {
            // Wait for materials
            (
                false,
                State {
                    time_used: new_time_used,
                    inventory: upd_inventory.clone(),
                    robots: old_state.robots.clone(),
                    next_build: Build::Geode,
                    builds: old_state.builds.clone(),
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
            Build::Obsidian => true,
            Build::Any
                if old_state.robots.ore > 0
                    && old_state.robots.clay > 0
                    && (old_state.robots.obsidian as MineralQty)
                        < sim_parms.blueprint.geode_robot_obsidian =>
            {
                true
            }
            _ => false,
        } {
            let next_state = if old_state.inventory.ore >= sim_parms.blueprint.obsidian_robot_ore
                && old_state.inventory.clay >= sim_parms.blueprint.obsidian_robot_clay
            {
                // Build the robot
                let mut new_builds = old_state.builds.clone();
                new_builds.push((old_state.time_used, Build::Obsidian));

                let mut new_inventory = upd_inventory.clone();
                new_inventory.ore -= sim_parms.blueprint.obsidian_robot_ore;
                new_inventory.clay -= sim_parms.blueprint.obsidian_robot_clay;

                let mut new_robots = old_state.robots.clone();
                new_robots.obsidian += 1;

                State {
                    time_used: new_time_used,
                    inventory: new_inventory,
                    robots: new_robots,
                    next_build: Build::Any,
                    builds: new_builds,
                }
            } else {
                // Wait for materials
                State {
                    time_used: new_time_used,
                    inventory: upd_inventory.clone(),
                    robots: old_state.robots.clone(),
                    next_build: Build::Obsidian,
                    builds: old_state.builds.clone(),
                }
            };

            simulate_iter(next_state, sim_parms, result);
        }

        // Clay robot
        if match old_state.next_build {
            Build::Clay => true,
            Build::Any
                if (old_state.robots.clay as MineralQty)
                    < sim_parms.blueprint.obsidian_robot_clay =>
            {
                true
            }
            _ => false,
        } {
            let next_state = if old_state.inventory.ore >= sim_parms.blueprint.clay_robot_ore {
                // Build the robot
                let mut new_builds = old_state.builds.clone();
                new_builds.push((old_state.time_used, Build::Clay));

                let mut new_inventory = upd_inventory.clone();
                new_inventory.ore -= sim_parms.blueprint.clay_robot_ore;

                let mut new_robots = old_state.robots.clone();
                new_robots.clay += 1;

                State {
                    time_used: new_time_used,
                    inventory: new_inventory,
                    robots: new_robots,
                    next_build: Build::Any,
                    builds: new_builds,
                }
            } else {
                // Wait for materials
                State {
                    time_used: new_time_used,
                    inventory: upd_inventory.clone(),
                    robots: old_state.robots.clone(),
                    next_build: Build::Clay,
                    builds: old_state.builds.clone(),
                }
            };

            simulate_iter(next_state, sim_parms, result);
        }

        // Ore robot
        if match old_state.next_build {
            Build::Ore => true,
            Build::Any if (old_state.robots.ore as MineralQty) < sim_parms.max_ore => true,
            _ => false,
        } {
            let next_state = if old_state.inventory.ore >= sim_parms.blueprint.ore_robot_ore {
                // Build the robot
                let mut new_builds = old_state.builds;
                new_builds.push((old_state.time_used, Build::Ore));

                let mut new_inventory = upd_inventory;
                new_inventory.ore -= sim_parms.blueprint.ore_robot_ore;

                let mut new_robots = old_state.robots;
                new_robots.ore += 1;

                State {
                    time_used: new_time_used,
                    inventory: new_inventory,
                    robots: new_robots,
                    next_build: Build::Any,
                    builds: new_builds,
                }
            } else {
                // Wait for materials
                State {
                    time_used: new_time_used,
                    inventory: upd_inventory,
                    robots: old_state.robots.clone(),
                    next_build: Build::Ore,
                    builds: old_state.builds,
                }
            };

            simulate_iter(next_state, sim_parms, result);
        }
    }
}
