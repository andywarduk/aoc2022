use crate::cube::Cube;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Face {
    axis: Axis,
    pub pos: isize,
    other: [isize; 2],
}

impl Face {
    pub fn new(axis: Axis, pos: isize, other: [isize; 2]) -> Self {
        Self { axis, pos, other }
    }

    pub fn to_cube_at(&self, pos: isize) -> Cube {
        match self.axis {
            Axis::X => Cube::new(pos, self.other[0], self.other[1]),
            Axis::Y => Cube::new(self.other[0], pos, self.other[1]),
            Axis::Z => Cube::new(self.other[0], self.other[1], pos),
        }
    }

    pub fn axis(&self) -> &Axis {
        &self.axis
    }
}
