use crate::{utils::{charmap::*, directions::Direction, point::Point}, Application};



impl Application {
    pub fn day4(self) {
        let char_map = CharMap::new_from_input(&self.input);
        if self.args.part == 1 {
            self.d4p1(char_map);
        } else {
            self.d4p2(char_map);
        }
    }

    fn d4p1(self, char_map: CharMap) {
        let answer = char_map.occurances("XMAS").len();
        println!("{answer}");
    }

    fn d4p2(self, char_map: CharMap) {
        let mut answer = 0;
        for row in 0..char_map.map.len() {
            for column in 0..char_map.map[row].len() {
                let pos = Point::new(column as isize, row as isize);
                if char_map.map[row][column] == 'A' {
                    if char_map.find_from_char("MAS", 1, &pos)
                        .iter()
                        .filter(
                            |dir| match dir {
                                Direction::NorthEast => true,
                                Direction::NorthWest => true,
                                Direction::SouthEast => true,
                                Direction::SouthWest => true,
                                _ => false
                            })
                        .collect::<Vec<&Direction>>()
                        .len() >= 2 {
                        answer +=1
                    }
                    
                }
            }
        }
        println!("{answer}");
    }
}


