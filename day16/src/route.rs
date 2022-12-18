use crate::{floydwarshall::FloydWarshall, xref::XRef, START_ROOM};

// Structure to hold a route
#[derive(Debug, Default, Clone)]
pub struct Route(Vec<u8>);

impl Route {
    /// Add to the route
    pub fn add(&mut self, loc: u8) {
        self.0.push(loc)
    }

    /// Return bitmask of valves
    pub fn mask(&self) -> u64 {
        self.0.iter().fold(0, |acc, v| acc | (1 << v))
    }

    /// Pretty prints the route
    pub fn print(&self, xref: &XRef, map: &FloydWarshall) {
        println!("{}", self.pretty(xref, map))
    }

    /// Formats the route
    pub fn pretty(&self, xref: &XRef, map: &FloydWarshall) -> String {
        let mut result = START_ROOM.to_string();

        let mut loc = xref.index_for_valve(START_ROOM);

        for p in self.0.iter() {
            for r in map.path_idx(loc, *p).iter().skip(1) {
                result += &format!(" {}", xref.valve_for_index(*r));
            }

            result += " (open)";

            loc = *p;
        }

        result
    }
}
