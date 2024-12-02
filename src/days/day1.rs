use crate::Application;
use std::collections::BTreeMap;

impl Application {
    pub fn day1(self) {
        if self.args.part == 1 {
            self.d1p1();
        } else {
            self.d1p2();
        }
    }

    fn d1p1(self) {
        let mut left = Vec::<i32>::new();
        let mut right = Vec::<i32>::new();
        for line in &self.input {
            let mut split = line.split_whitespace();
            left.push(split.next().unwrap().parse().unwrap());
            right.push(split.next().unwrap().parse().unwrap());
        }

        left.sort();
        right.sort();
        let mut answer = 0;

        for i in 0..left.len() {
            answer += (left[i] - right[i]).abs()
        }
        println!("{answer}");
    }

    fn d1p2(self) {
        let mut left = Vec::<i32>::new();
        let mut right = Vec::<i32>::new();
        for line in &self.input {
            let mut split = line.split_whitespace();
            left.push(split.next().unwrap().parse().unwrap());
            right.push(split.next().unwrap().parse().unwrap());
        }

        let mut lookup = BTreeMap::new();
        let mut answer = 0;
        
        for left_number in left {
            if let Some(value) = lookup.get(&left_number) {
                answer += value;
            } else {
                let mut occurance = 0;
                let mut to_remove = Vec::new();
                for i in 0..right.len() {
                    if right[i] == left_number {
                        occurance += 1;
                        to_remove.push(i);
                    }
                }
                to_remove.reverse();
                for index in to_remove {
                    right.remove(index);
                }
                let value = occurance * left_number;
                lookup.insert(left_number, value);
                answer += value;
            }
        }
        println!("{answer}");
    }
}
