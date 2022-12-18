use std::cmp::Ordering;

use crate::{floydwarshall::FloydWarshall, route::Route, InputEnt};

/// Walks all routes
pub fn walk<F>(input: &[InputEnt], dist_map: &FloydWarshall, state: WalkState, cb: &mut F)
where
    F: FnMut(&WalkState),
{
    if !state.valves.is_empty() {
        // TODO NEEDED?
        // Build list of choices (distance, rate, valve element)
        let mut choices = state
            .valves
            .iter()
            .enumerate()
            .map(|(velem, v)| {
                (
                    dist_map.dist_idx(state.vno, *v),
                    input[*v as usize].rate,
                    velem,
                )
            })
            .collect::<Vec<_>>();

        // Sort by distance ascending then rate descending
        choices.sort_by(|a, b| match a.0.cmp(&b.0) {
            Ordering::Equal => b.1.cmp(&a.1),
            c => c,
        });

        for (dist, rate, velem) in choices {
            if state.time_left > dist {
                let next_vno = state.valves[velem];

                let mut next_valves = state.valves.clone();
                next_valves.swap_remove(velem);

                let time_spent = dist as usize + 1;

                let mut route = state.route.clone();
                route.add(next_vno);

                let next_state = WalkState {
                    vno: next_vno,
                    valves: next_valves,
                    rate: state.rate + rate as usize,
                    released: state.released + (state.rate * time_spent),
                    time_left: state.time_left - time_spent as u8,
                    route,
                };

                cb(&next_state);

                if !state.valves.is_empty() {
                    walk(input, dist_map, next_state, cb);
                }
            }
        }
    }
}

/// State for each stage of walk
pub struct WalkState {
    vno: u8,
    valves: Vec<u8>,
    pub rate: usize,
    pub released: usize,
    pub time_left: u8,
    pub route: Route,
}

impl WalkState {
    /// Creates new walk state
    pub fn new(vno: u8, valves: Vec<u8>, time_left: u8) -> Self {
        Self {
            vno,
            valves,
            rate: 0,
            released: 0,
            time_left,
            route: Route::default(),
        }
    }
}
