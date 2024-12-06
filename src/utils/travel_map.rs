pub use super::{directions::*, point::Point};

#[allow(unused, dead_code)]

pub struct TravelMap {
    pub map: Vec<Vec<char>>,
    pub location: Point,
    pub direction: Direction
}

impl TravelMap {
    pub fn new_from_input(input: &Vec<String>, start: char, direction: Direction) -> Self {
        let mut map: Vec<Vec<char>> = Vec::new();
        let mut location: Point = Point::new(0, 0);
        for line_num in 0..input.len() {
            map.push(Vec::new());
            for (column, character) in input[line_num].chars().enumerate() {
                if character == start {
                    location = Point::new(column as isize, line_num as isize);
                }
                map[line_num].push(character);
            }
        }
        return Self { map, location, direction };
    }

    pub fn adjacent(&self) -> Option<char> {
        if self.out_of_bounds(&self.direction) {
            return None;
        }
        let target = self.location + self.direction.to_coordinates();
        return Some(self.map[target.y() as usize][target.x() as usize]);
    }

    pub fn move_by_one(&mut self) {
        self.location = self.location + self.direction.to_coordinates()
    }

    pub fn turn_right(&mut self) {
        self.direction = self.direction.turn90();
    }

    fn out_of_bounds(&self, direction: &Direction) -> bool {
        match direction {
            Direction::North => { if self.location.y() == 0 { return true } },
            Direction::NorthEast => { if self.location.y() == 0 || self.location.x() == self.right_bound_at_num((self.location.y() - 1) as usize) { return true } },
            Direction::East => { if self.location.x() == self.right_bound_at_pos() { return true } },
            Direction::SouthEast => { if self.location.x() == self.right_bound_at_num((self.location.y() + 1) as usize) || self.location.y() == self.bottom_bound() - 1 { return true } },
            Direction::South => { if self.location.y() == self.bottom_bound() - 1 { return  true } },
            Direction::SouthWest => { if self.location.y() == self.bottom_bound() - 1 || self.location.x() == 0 { return true } },
            Direction::West => { if self.location.x() == 0 { return true } },
            Direction::NorthWest => { if self.location.y() == 0 || self.location.x() == 0 { return true } }
        }
        return false;
    }

    fn bottom_bound(&self) -> isize {
        return self.map.len() as isize;
    }

    fn right_bound_at_pos(&self) -> isize {
        return self.map[self.location.y() as usize].len() as isize;
    }

    fn right_bound_at_num(&self, line: usize) -> isize {
        return self.map[line].len() as isize;
    }
}
