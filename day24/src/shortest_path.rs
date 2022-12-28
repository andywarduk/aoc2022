use std::collections::{HashMap, HashSet, VecDeque};

use crate::{map::Map, pos::Pos};

pub fn shortest_path<F>(
    map: &Map,
    start_time: usize,
    start_pos: Pos,
    end_pos: Pos,
    mut state_cb: F,
) -> Option<usize>
where
    F: FnMut(&State, &HashMap<Pos, usize>, &VecDeque<State>, &Pos),
{
    let mut states = VecDeque::new();

    states.push_back(State {
        time: start_time,
        pos: start_pos.clone(),
        path: vec![start_pos.clone()],
    });

    let mut visited = HashSet::new();

    while let Some(state) = states.pop_front() {
        // Get blizzard positions
        let blizzards = &map.blizzards[state.time % map.blizzards.len()];

        // Call state callback function
        state_cb(&state, blizzards, &states, &end_pos);

        // Finished?
        if state.pos == end_pos {
            return Some(state.time - 1);
        }

        // Calculate valid next positions
        let mut add_next = |x, y| {
            let pos = Pos { x, y };

            if ((x > 0 && x <= map.width && y > 0 && y <= map.height)
                || pos == end_pos
                || pos == start_pos)
                && !blizzards.contains_key(&pos)
                && !visited.contains(&(pos.clone(), state.time + 1))
            {
                let mut next_state = state.clone();

                next_state.time = state.time + 1;
                next_state.pos = pos.clone();
                next_state.path.push(pos.clone());

                states.push_back(next_state);
                visited.insert((pos, state.time + 1));
            }
        };

        add_next(state.pos.x + 1, state.pos.y);
        add_next(state.pos.x, state.pos.y + 1);
        add_next(state.pos.x - 1, state.pos.y);
        if state.pos.y > 0 {
            add_next(state.pos.x, state.pos.y - 1);
        }
        add_next(state.pos.x, state.pos.y);
    }

    None
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub time: usize,
    pub pos: Pos,
    pub path: Vec<Pos>,
}
