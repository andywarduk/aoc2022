use super::pos::Pos;

/// Positions with distance for the work queue
pub struct WorkItem<T> {
    pub pos: Pos,
    pub data: T,
}

impl<T> WorkItem<T> {
    pub fn new(pos: Pos, data: T) -> Self {
        Self { pos, data }
    }
}
