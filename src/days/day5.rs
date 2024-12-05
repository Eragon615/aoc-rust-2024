use std::collections::BTreeMap;

use crate::Application;

impl Application {
    pub fn day5(self) {
        let (rules, updates) = parser(&self.input);
        if self.args.part == 1 {
            self.d5p1(rules, updates);
        } else {
            self.d5p2();
        }
    }

    fn d5p1(self, rules: BTreeMap<usize, Vec<usize>>, updates: Vec<Vec<usize>>) {
        let mut answer = 0;
        for update in updates {
            if entry_is_valid(&rules, &update) {
                let midpoint = (update.len() - 1) / 2;
                answer += update[midpoint];
            }
        }
        println!("{answer}");
    }

    fn d5p2(self) {
        let mut answer = 0;

        println!("{answer}");
    }
}

fn parser(input: &Vec<String>) -> (BTreeMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut line_num: usize = 0;
    let mut btree: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut out_vec = Vec::new();
    while input[line_num] != "" {
        let mut split_line = input[line_num].split('|');
        let mut left = 0;
        let mut right = 0;
        if let Some(num) = split_line.next() {
            left = num.parse().expect("You didn't parse a number");
        }
        if let Some(num) = split_line.next() {
            right = num.parse().expect("You didn't parse a number");
        }
        btree.entry(left).and_modify(|val| val.push(right)).or_insert(vec![right]);
        line_num += 1;
    }
    line_num += 1;        
    let mut counter = 0;
    while input[line_num] != "" {

        let mut split_line = input[line_num].split(',');
        out_vec.push(Vec::new());
        while let Some(num) = split_line.next() {
            out_vec[counter].push(num.parse().expect("There was a non-number in the second half."));
        }
        counter += 1;
        line_num += 1;
    }
    println!("DEBUG: {btree:?}");
    println!("DEBUG: {out_vec:?}");
    return (btree, out_vec);
}

fn entry_is_valid(rules: &BTreeMap<usize, Vec<usize>>, update: &Vec<usize>) -> bool {
    let mut response = true;
    let mut update = update.clone();
    update.reverse();
    println!("{update:?}");
    for (idx, number) in update.iter().enumerate() {
        let rules_lookup = rules.get(number);
        if let Some(num_rules) = rules_lookup {
            for next_page in (idx + 1)..update.len() {
                if num_rules.contains(&next_page) {
                    response = false; 
                }
            }
        }
    }
    return response;
}