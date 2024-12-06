use crate::utils::{strings::split_from, directions::*};

#[derive(Debug)]
pub struct CharMap {
    pub map: Vec<Vec<char>>
}

impl CharMap {
    pub fn new_from_input(input: &Vec<String>) -> CharMap {
        let mut map: Vec<Vec<char>> = Vec::new();
        for line in input {
            map.push(line.chars().collect());
        }
        return CharMap {
            map
        }
    }

    fn right_bound(&self) -> usize {
        self.map[0].len()
    }

    fn bottom_bound(&self) -> usize {
        self.map.len()
    }

    pub fn occurances(&self, search: &str) -> Vec<Direction> {
        let first_letter: char = search.chars().nth(0).expect("No first letter in search.");
        let mut output = Vec::new();
        for row in 0..self.right_bound() {
            for column in 0..self.bottom_bound() {
                if self.map[row][column] == first_letter {
                    let pos = Point::new(column as isize, row as isize);
                    output.append(&mut self.find_adjacents(search, &pos));
                }
            }
        }
        return output;
    }

    fn find_adjacents(&self, search: &str, start_pos: &Point) -> Vec<Direction> {
        let mut output = Vec::new();
        let length = search.len() - 1;
        for direction in Direction::all() {
            if !self.out_of_bounds(&direction, start_pos, length) {
                let word = self.collect_word(&direction, start_pos, length);
                if word.as_str() == search {
                    output.push(direction);
                }
            }
        }
        return output;
    }

    pub fn find_from_char(&self, search: &str, from: usize, pos: &Point) -> Vec<Direction> {
        let mut output = Vec::new();
        let (left, right) = split_from(search, from).expect("Don't misuse split_from");
        let mut word: String;
        for direction in Direction::all() {
            if !self.out_of_bounds(&direction, pos, left.len() - 1) 
            && !self.out_of_bounds(&direction.mirrored(), pos, right.len() - 1) {
                word = self.collect_word(&direction, pos, left.len() - 1).chars().rev().collect();
                if word != left {
                    continue;
                }
                word = self.collect_word(&direction.mirrored(), pos, right.len() - 1);
                if word != right {
                    continue;
                }
                output.push(direction);
            }
        }
        return output;
    }

    fn out_of_bounds(&self, direction: &Direction, pos: &Point, length: usize) -> bool {
        match direction {
            Direction::North => { if length > pos.y() as usize { return true } },
            Direction::NorthEast => { if length > pos.y() as usize || (length + pos.x() as usize) >= self.right_bound() { return true } },
            Direction::East => { if (length + pos.x() as usize) >= self.right_bound() { return true } },
            Direction::SouthEast => { if (length + pos.x() as usize) >= self.right_bound() || (length + pos.y() as usize) >= self.bottom_bound() { return true } },
            Direction::South => { if (length + pos.y() as usize) >= self.bottom_bound() { return  true } },
            Direction::SouthWest => { if (length + pos.y() as usize) >= self.bottom_bound() || length > pos.x() as usize { return true } },
            Direction::West => { if length > pos.y() as usize { return true } },
            Direction::NorthWest => { if length > pos.y() as usize || length > pos.x() as usize { return true } }
        }
        return false;
    }

    fn collect_word(&self, direction: &Direction, start_pos: &Point, length: usize) -> String {
        let mut output = String::new();
        let mut pos = start_pos.clone();
            for _ in 0..=length {
                pos = pos + direction.to_coordinates();
                output.push(self.map[pos.y() as usize][pos.x() as usize]);
            }
        return output;
    }

}