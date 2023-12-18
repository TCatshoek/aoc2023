use glam::IVec2;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, Ord, PartialOrd)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn from_udlr(input: &str) -> Self {
        match input {
            "U" => Direction::North,
            "D" => Direction::South,
            "R" => Direction::East,
            "L" => Direction::West,
            x => panic!("Invalid direction: {}", x)
        }
    }

    pub fn as_delta(&self) -> IVec2 {
        match self {
            Direction::North => IVec2::new(0, -1),
            Direction::East => IVec2::new(1, 0),
            Direction::South => IVec2::new(0, 1),
            Direction::West => IVec2::new(-1, 0)
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East
        }
    }
}