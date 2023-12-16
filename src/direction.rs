use glam::IVec2;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn as_delta(&self) -> IVec2 {
        match self {
            Direction::North => IVec2::new(0, -1),
            Direction::East => IVec2::new(1, 0),
            Direction::South => IVec2::new(0, 1),
            Direction::West => IVec2::new(-1, 0)
        }
    }
}