pub use super::{directions::*, point::Point};

#[allow(unused, dead_code)]

#[derive(Debug, Clone)]
pub struct TravelMap {
    pub map: Vec<Vec<char>>,
    pub location: Point,
    pub direction: Direction
}

#[allow(unused, dead_code)]
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

    pub fn adjacent(&self, direction: &Direction) -> Option<char> {
        let target = self.location + direction.to_coordinates();
        if let Some(row) =  self.map.get(target.y() as usize) {
            if let Some(tile) = row.get(target.x() as usize) {
                return Some(*tile);
            }
        }
        return None;
    }

    pub fn move_by_one(&mut self) {
        self.location = self.location + self.direction.to_coordinates()
    }

    pub fn turn_right(&mut self) {
        self.direction = self.direction.turn90();
    }

    fn char_at_pos(&self, pos: Point) -> Option<char> {
        if let Some(row) =  self.map.get(pos.y() as usize) {
            if let Some(tile) = row.get(pos.x() as usize) {
                return Some(*tile);
            }
        }
        return None;
    }

    pub fn ray_cast_until(&self, direction: &Direction, object: char) -> Vec<Point> {
        let mut cast_pos = self.location;
        let mut output = Vec::new();
        while let Some(tile) = self.char_at_pos(cast_pos) {
            if tile != object {
                output.push(cast_pos);
                cast_pos = cast_pos + direction.to_coordinates();
            } else {
                break;
            }
        }
        return output;
    }
}
