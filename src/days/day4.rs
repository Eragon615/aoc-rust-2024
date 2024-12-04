use crate::{Application, utils::charmap::*};



impl Application {
    pub fn day4(self) {
        let char_map = CharMap::new_from_input(&self.input);
        if self.args.part == 1 {
            self.d4p1(char_map);
        } else {
            self.d4p2();
        }
    }

    fn d4p1(self, char_map: CharMap) {
        let answer = char_map.occurances("XMAS");
        println!("{answer}");
    }

    fn d4p2(self) {

    }
}


