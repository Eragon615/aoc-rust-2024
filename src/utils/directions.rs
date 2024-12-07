pub use crate::utils::point::*;

#[allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East, 
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        let mut output = Vec::new();
        output.push(Direction::North);
        output.push(Direction::NorthEast);
        output.push(Direction::East);
        output.push(Direction::SouthEast);
        output.push(Direction::South);
        output.push(Direction::SouthWest);
        output.push(Direction::West);
        output.push(Direction::NorthWest);
        return output;
    }

    pub fn to_coordinates(&self) -> Point {
        match self {
            Direction::North => Point::new(0, -1),
            Direction::NorthEast => Point::new(1, -1),
            Direction::East => Point::new(1, 0),
            Direction::SouthEast => Point::new(1, 1),
            Direction::South => Point::new(0, 1),
            Direction::SouthWest => Point::new(-1, 1),
            Direction::West => Point::new(-1, 0),
            Direction::NorthWest => Point::new(-1, -1)
        }
    }

    pub fn mirrored(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::NorthEast => Direction::SouthWest,
            Direction::East => Direction::West,
            Direction::SouthEast => Direction::NorthWest,
            Direction::South => Direction::North,
            Direction::SouthWest => Direction::NorthEast,
            Direction::West => Direction::East,
            Direction::NorthWest => Direction::SouthEast
        }
    }

    pub fn turn90(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::NorthEast => Direction::SouthEast,
            Direction::East => Direction::South,
            Direction::SouthEast => Direction::SouthWest,
            Direction::South => Direction::West,
            Direction::SouthWest => Direction::NorthWest,
            Direction::West => Direction::North,
            Direction::NorthWest => Direction::NorthEast
        }
    }
}
