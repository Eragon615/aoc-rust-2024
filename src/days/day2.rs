use crate::Application;


impl Application {
    pub fn day2(self) {
        if self.args.part == 1 {
            self.d2p1();
        } else {
            self.d2p2();
        }
    }

    fn d2p1(self) {
        let mut levels: Vec<Vec<i32>> = Vec::new();
        for i in 0..self.input.len() {
            levels.push(Vec::new());
            let mut line = self.input[i].split_whitespace();
            while let Some(value) = line.next() {
                levels[i].push(value.parse().unwrap());
            }
        }
        let mut answer = 0; 
        for reading in levels {
            let mut valid = true;
            let mut sign = None;
            for i in 1..reading.len() {
                let variance = reading[i] - reading[i-1];
                match sign {
                    None => {sign = Some(variance.signum())},
                    Some(pos_neg) => {
                        if pos_neg != variance.signum() {
                            valid = false;
                        };
                    }
                };
                if variance.abs() != 1 && variance.abs() != 2 && variance.abs() != 3 {
                    valid = false;
                }
            }
            if valid {
                answer += 1;
            }
        }
        println!("{answer}");
    }

    fn d2p2(self) {
        let mut levels: Vec<Vec<i32>> = Vec::new();
        for i in 0..self.input.len() {
            levels.push(Vec::new());
            let mut line = self.input[i].split_whitespace();
            while let Some(value) = line.next() {
                levels[i].push(value.parse().unwrap());
            }
        }
        let mut answer = 0; 
        for reading in levels {
            let mut bad_level = 0;
            let mut positives = 0;
            let mut negatives = 0;
            let mut valid = true;
            for i in 1..reading.len() {
                let variance = reading[i] - reading[i-1];
                if variance.signum() == 1 {
                    positives += 1;
                } else {
                    negatives += 1;
                }
                if variance.abs() != 1 && variance.abs() != 2 && variance.abs() != 3 {
                    bad_level += 1;
                }
            }
            let sign_change:i32 = positives - negatives;
            if sign_change.abs() == reading.len() as i32 && bad_level <= 1 {
            } else if reading.len() as i32 - sign_change.abs() == 1  && bad_level == 0 {
            } else {
                valid = false;
            }
            println!("{reading:?} {sign_change} {bad_level}");
            if valid {
                answer += 1;
            }
        }
        println!("{answer}");
    }
}
