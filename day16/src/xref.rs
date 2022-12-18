use std::collections::HashMap;

use crate::InputEnt;

/// Cross reference table from string to element for each room
pub struct XRef {
    vimap: HashMap<String, u8>,
    ivmap: HashMap<u8, String>,
}

impl XRef {
    /// Create xref table from input
    pub fn new(input: &[InputEnt]) -> Self {
        let vimap = input
            .iter()
            .enumerate()
            .map(|(i, ent)| (ent.valve.clone(), i as u8))
            .collect::<HashMap<_, _>>();

        let ivmap = input
            .iter()
            .enumerate()
            .map(|(i, ent)| (i as u8, ent.valve.clone()))
            .collect::<HashMap<_, _>>();

        Self { vimap, ivmap }
    }

    /// Look up index for string room
    pub fn index_for_valve(&self, valve: &str) -> u8 {
        *self.vimap.get(valve).unwrap()
    }

    /// Return string room for index
    pub fn valve_for_index(&self, index: u8) -> &String {
        self.ivmap.get(&index).unwrap()
    }
}
