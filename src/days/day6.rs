use std::collections::HashSet;

use crate::{utils::{directions::Direction, travel_map::*}, Application};


impl Application {
    pub fn day6(self) {
        if self.args.part == 1 {
            self.d6p1();
        } else {
            self.d6p2();
        }
    }

    fn d6p1(self) {
        let mut travel_map = TravelMap::new_from_input(&self.input, '^', Direction::North);
        let mut travel_history = HashSet::new();
        _ = travel_history.insert(travel_map.location);
        while let Some(tile) = travel_map.adjacent(&travel_map.direction) {
            if tile == '#' {
                travel_map.turn_right();
            } else {
                _ = travel_history.insert(travel_map.location);
                travel_map.move_by_one();
            }
        }
        _ = travel_history.insert(travel_map.location);
        println!("{}", travel_history.len());
    }

    fn d6p2(self) {
        let mut travel_map = TravelMap::new_from_input(&self.input, '^', Direction::North);        
        let mut travel_history = HashSet::new();
        let mut answers = HashSet::new();
        _ = travel_history.insert((travel_map.location, travel_map.direction));
        while let Some(tile) = travel_map.adjacent(&travel_map.direction) {
            // println!("MAIN: I'm at {:?}, facing {:?} and there's a {tile} in front of me.", travel_map.location, travel_map.direction);
            if tile == '#' {
                travel_map.turn_right();
                // println!("MAIN: I'm turning {:?}, and adding {:?} to my history.", travel_map.direction, travel_map.location);
                continue;
            }
            if tile != '^' {
                let obstruction_at = travel_map.location + travel_map.direction.to_coordinates();
                // println!("MAIN: I'm placing an object at {:?}", obstruction_at);

                let mut new_map = travel_map.clone();
                new_map.map[obstruction_at.y() as usize][obstruction_at.x() as usize] = '#';
                // println!("MAIN: I'm checking if that obstruction results in a loop.");
                if check_loop(new_map, travel_history.clone()) {
                    let obstruction_at = travel_map.location + travel_map.direction.to_coordinates();
                    if answers.insert(obstruction_at) {
                        // println!("MAIN: I see a new loop at {} {}.", obstruction_at.x(), obstruction_at.y());
                    }
                }
            }
            _ = travel_history.insert((travel_map.location, travel_map.direction));

            travel_map.move_by_one();
        }
        println!("{}", answers.len());
    }
}

fn check_loop(mut map: TravelMap, mut history: HashSet<(Point, Direction)>) -> bool {
    while let Some(tile) = map.adjacent(&map.direction) {
        // println!("  CHCK: I'm at {:?}, facing {:?}, and there's a {tile} in front of me.", map.location, map.direction);
       if tile == '#' {
            // println!("  CHCK: I'm turning.");
            map.turn_right();
            continue;
        }
        if !history.insert((map.location, map.direction)) {
            // println!("  CHCK: Couldn't add {:?}, must be in a loop.", map.location);
            return true;
        }
        // println!("  CHCK: I added {:?} to my temp history. Moving.", map.location);
        map.move_by_one();
    }
    // println!("  CHCK: My next step takes me off the map. This location doesn't cause a loop.");
    return false;
}
