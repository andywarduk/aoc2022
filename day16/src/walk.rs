use crate::{floydwarshall::FloydWarshall, route::Route, InputEnt};

/// Walks all routes
pub fn walk<F>(input: &[InputEnt], dist_map: &FloydWarshall, state: WalkState, cb: &mut F)
where
    F: FnMut(&WalkState),
{
    // Process each valve in turn
    for vno in state.valves.iter() {
        // Get distance
        let dist = dist_map.dist_idx(state.vno, *vno);

        // Get rate at destination
        let rate = input[*vno as usize].rate;

        // Enough time left?
        if state.time_left > dist {
            // Create next valves vector
            let next_valves = state
                .valves
                .iter()
                .filter(|v| **v != *vno)
                .copied()
                .collect::<Vec<_>>();

            // Calculate time spent
            let time_spent = dist as usize + 1;

            // Create next route vector
            let mut route = state.route.clone();
            route.add(*vno);

            // Build next state
            let next_state = WalkState {
                vno: *vno,
                valves: next_valves,
                rate: state.rate + rate as usize,
                released: state.released + (state.rate * time_spent),
                time_left: state.time_left - time_spent as u8,
                route,
            };

            // Call callback
            cb(&next_state);

            // More to process?
            if !next_state.valves.is_empty() {
                // Recurse
                walk(input, dist_map, next_state, cb);
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
