use crate::utils::strings::split_from;

#[derive(Debug)]
pub struct CharMap {
    pub map: Vec<Vec<char>>
}

#[derive(Debug)]
enum Direction {
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
    fn all() -> Vec<Direction> {
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

    fn to_coordinates(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::NorthEast => (1, -1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, 1),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, -1)
        }
    }

    fn mirrored(&self) -> Direction {
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

    pub fn occurances(&self, search: &str) -> usize {
        let first_letter: char = search.chars().nth(0).expect("No first letter in search.");
        let mut output = 0;
        for row in 0..self.right_bound() {
            for column in 0..self.bottom_bound() {
                if self.map[row][column] == first_letter {
                    output += self.find_adjacents(search, row, column);
                }
            }
        }
        return output;
    }

    fn find_adjacents(&self, search: &str, row: usize, column: usize) -> usize {
        let mut output = 0;
        let length = search.len() - 1;
        for direction in Direction::all() {
            if !self.out_of_bounds(&direction, row, column, length) {
                let word = self.collect_word(&direction, row, column, length);
                if word.as_str() == search {
                    output += 1;
                }
            }
        }
        return output;
    }

    pub fn find_from_char(&self, search: &str, from: usize, row: usize, column: usize) -> usize {
        let mut output = 0;
        let (left, right) = split_from(search, from).expect("Don't misuse split_from");
        let mut word = String::new();
        for direction in [Direction::NorthEast, Direction::NorthWest, Direction::SouthEast, Direction::SouthWest] {
            if !self.out_of_bounds(&direction, row, column, left.len() - 1) 
            && !self.out_of_bounds(&direction.mirrored(), row, column, right.len() - 1) {
                word = self.collect_word(&direction, row, column, left.len() - 1).chars().rev().collect();
                if word != left {
                    continue;
                }
                word = self.collect_word(&direction.mirrored(), row, column, right.len() - 1);
                if word != right {
                    continue;
                }
                output += 1;
            }
        }
        return output;
    }

    fn out_of_bounds(&self, direction: &Direction, row: usize, column: usize, length: usize) -> bool {
        match direction {
            Direction::North => { if length > row { return true } },
            Direction::NorthEast => { if length > row || (length + column) >= self.right_bound() { return true } },
            Direction::East => { if (length + column) >= self.right_bound() { return true } },
            Direction::SouthEast => { if (length + column) >= self.right_bound() || (length + row) >= self.bottom_bound() { return true } },
            Direction::South => { if (length + row) >= self.bottom_bound() { return  true } },
            Direction::SouthWest => { if (length + row) >= self.bottom_bound() || length > column { return true } },
            Direction::West => { if length > column { return true } },
            Direction::NorthWest => { if length > row || length > column { return true } }
        }
        return false;
    }

    fn collect_word(&self, direction: &Direction, row: usize, column: usize, length: usize) -> String {
        let mut output = String::new();
        let (x_dir, y_dir) = direction.to_coordinates();
            for count in 0..=length {
                let new_x_pos = (column as i32 + (x_dir * count as i32)) as usize;
                let new_y_pos = (row as i32 + (y_dir * count as i32)) as usize;
                output.push(self.map[new_y_pos][new_x_pos]);
            }
        return output;
    }

}