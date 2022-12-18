use crate::{xref::XRef, InputEnt};

// Distance and path map
pub struct FloydWarshall {
    dist: Vec<Vec<Option<u8>>>,
    next: Vec<Vec<Option<u8>>>,
}

impl FloydWarshall {
    /// Builds a new distance and path table from input
    pub fn new(input: &[InputEnt], xref: &XRef) -> Self {
        let dimension = input.len();

        let mut res = Self {
            dist: vec![vec![None; dimension]; dimension],
            next: vec![vec![None; dimension]; dimension],
        };

        for (u, ue) in input.iter().enumerate() {
            for t in ue.tunnels.iter() {
                let v = xref.index_for_valve(t);
                res.dist[u][v as usize] = Some(1);
                res.next[u][v as usize] = Some(v);
            }

            res.dist[u][u] = Some(0);
            res.next[u][u] = Some(u as u8);
        }

        for k in 0..dimension {
            for i in 0..dimension {
                for j in 0..dimension {
                    if res.dist[i][k].is_some() && res.dist[k][j].is_some() {
                        let sum = res.dist[i][k].unwrap() + res.dist[k][j].unwrap();
                        if res.dist[i][j].is_none() || res.dist[i][j].unwrap() > sum {
                            res.dist[i][j] = Some(sum);
                            res.next[i][j] = res.next[i][k];
                        }
                    }
                }
            }
        }

        res
    }

    /// Returns the distance between two locations given as strings
    pub fn dist_str(&self, xref: &XRef, from: &str, to: &str) -> u8 {
        let u = xref.index_for_valve(from);
        let v = xref.index_for_valve(to);

        self.dist_idx(u, v)
    }

    /// Returns the distance between two locations given as indexes
    pub fn dist_idx(&self, from: u8, to: u8) -> u8 {
        self.dist[from as usize][to as usize].unwrap()
    }

    /// Returns the path between two locations as strings
    pub fn path_str(&self, xref: &XRef, from: &str, to: &str) -> Vec<String> {
        let u = xref.index_for_valve(from);
        let v = xref.index_for_valve(to);

        self.path_idx(u, v)
            .into_iter()
            .map(|i| xref.valve_for_index(i).to_string())
            .collect::<Vec<_>>()
    }

    /// Returns the path between two locations as indexes
    pub fn path_idx(&self, mut from: u8, to: u8) -> Vec<u8> {
        let mut res = Vec::new();

        if self.next[from as usize][to as usize].is_some() {
            res.push(from);

            while from != to {
                from = self.next[from as usize][to as usize].unwrap();
                res.push(from)
            }
        }

        res
    }
}
