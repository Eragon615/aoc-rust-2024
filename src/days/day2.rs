use crate::Application;


impl Application {
    pub fn day2(self) {
        let mut reports: Vec<Vec<i32>> = Vec::new();
        for i in 0..self.input.len() {
            reports.push(Vec::new());
            let mut line = self.input[i].split_whitespace();
            while let Some(value) = line.next() {
                reports[i].push(value.parse().unwrap());
            }
        }
        if self.args.part == 1 {
            self.d2p1(reports);
        } else {
            self.d2p2(reports);
        }
    }

    fn d2p1(self, reports: Vec<Vec<i32>>) {
        let mut answer = 0; 
        for levels in reports {
            if is_valid(&levels) {
                answer += 1;
            }
        }
        println!("{answer}");
    }

    fn d2p2(self, reports: Vec<Vec<i32>>) {
        let mut answer = 0; 
        for levels in reports {
            if is_valid(&levels) {
                answer += 1;
            } else {
                for i in 0..levels.len() {
                    let mut new_levels = levels.clone();
                    new_levels.remove(i);
                    if is_valid(&new_levels) {
                        answer += 1;
                        break;
                    } 
                }
            }
        }
        println!("{answer}");
    }
}

fn is_valid(levels: &Vec<i32>) -> bool {
    let mut sign = None;
    for i in 1..levels.len() {
        let variance = levels[i] - levels[i-1];
        match sign {
            None => {sign = Some(variance.signum())},
            Some(pos_neg) => {
                if pos_neg != variance.signum() {
                    return false;
                };
            }
        };
        if variance.abs() != 1 && variance.abs() != 2 && variance.abs() != 3 {
            return false;
        }
    }
    return true;
}