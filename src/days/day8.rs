use std::{collections::{BTreeMap, HashSet}, vec};

use crate::{utils::{charmap::CharMap, point::Point}, Application};

impl Application {
    pub fn day8(self) {
        if self.args.part == 1 {
            self.d8p1();
        } else {
            self.d8p2();
        }
    }

    fn d8p1(self) {
        let mut btree = BTreeMap::new();
        let mut hash = HashSet::new();
        let roof = CharMap::new_from_input(&self.input);
        roof.gather_points(&mut btree, &vec!['.']);
        for (_, points) in btree {
            let mut pairs = Vec::new();
            for i in 0..points.len() - 1 {
                for j in i+1..points.len() {
                    pairs.push((points[i], points[j]));
                }
            }
            for pair in pairs {
                let distance = pair.0 - pair.1;
                let mut new_point = pair.1 - distance;
                if roof.is_in_bounds(new_point) {
                    _ = hash.insert(new_point);
                }
                new_point = pair.0 - (distance * Point::new(-1, -1));
                if roof.is_in_bounds(new_point) {
                    _ = hash.insert(new_point);
                }
            }
        }
        println!("{}", hash.len());
    }

    fn d8p2(self) {}
}

// use point math to generate new points 
// add new points to hashset to eliminate duplicates