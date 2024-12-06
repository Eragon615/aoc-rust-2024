use std::collections::HashSet;

use crate::{utils::{directions::Direction, travel_map::{self, *}}, Application};


impl Application {
    pub fn day6(self) {
        if self.args.part == 1 {
            self.d6p1();
        } else {
            self.d6p2();
        }
    }

    fn d6p1(self) {
        let mut travel_history = HashSet::new();
        let mut travel_map = TravelMap::new_from_input(&self.input, '^', Direction::North);
        while let Some(tile) = travel_map.adjacent() {
            if tile == '#' {
                travel_map.turn_right();
            } else {
                _ = travel_history.insert(travel_map.location.clone());
                travel_map.move_by_one();
            }
        }
        println!("{}", travel_history.len() + 1);
    }

    fn d6p2(self) {
        let mut answer = 0;

        println!("{answer}");
    }
}
