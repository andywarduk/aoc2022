use crate::face::{Axis, Face};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn new_from_adj(c: &Cube, xa: isize, ya: isize, za: isize) -> Self {
        Self::new(c.x + xa, c.y + ya, c.z + za)
    }

    pub fn to_faces(&self) -> Vec<Face> {
        vec![
            Face::new(Axis::X, self.x, [self.y, self.z]),
            Face::new(Axis::X, self.x + 1, [self.y, self.z]),
            Face::new(Axis::Y, self.y, [self.x, self.z]),
            Face::new(Axis::Y, self.y + 1, [self.x, self.z]),
            Face::new(Axis::Z, self.z, [self.x, self.y]),
            Face::new(Axis::Z, self.z + 1, [self.x, self.y]),
        ]
    }
}
