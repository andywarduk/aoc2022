use crate::{floydwarshall::FloydWarshall, xref::XRef, START};

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
        let mut loc = xref.index_for_valve(START);

        print!("{START}");

        for p in self.0.iter() {
            for r in map.path_idx(loc, *p).iter().skip(1) {
                print!(" {}", xref.valve_for_index(*r));
            }

            print!(" (open)");

            loc = *p;
        }

        println!();
    }
}
