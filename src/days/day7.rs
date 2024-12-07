use crate::Application;

struct Equation {
    outcome: u64,
    input: EquationInput
}

struct EquationInput(Vec<u64>);

impl Application {
    pub fn day7(self) {
        if self.args.part == 1 {
            self.d7p1();
        } else {
            self.d7p2();
        }
    }

    fn d7p1(self) {
        let mut answer = 0;
        let equations = parser(self.input);
        for equation in equations {
            if equation.is_valid() {
                answer += equation.outcome;
            }
        }
        println!("{answer}");
    }

    fn d7p2(self) {
        let mut answer = 0;
        let equations = parser(self.input);
        for equation in equations {
            if equation.is_valid_p2() {
                answer += equation.outcome;
            }
        }
        println!("{answer}");
    }
}


impl Equation {
    fn new_from_input(input: String) -> Self {
        let mut splits = input.split(':');
        let outcome = splits.next().expect("This line was formatted wrong").parse().expect("Some letter snuck in.");
        let mut rhs = splits.next().expect("The right hand side was messed up.").split_whitespace();
        let mut eq_inputs = Vec::new();
        while let Some(num) = rhs.next() {
            let num_as_u64 = num.parse().expect("A letter got into the rhs.");
            eq_inputs.push(num_as_u64);
        }
        Self { outcome, input: EquationInput(eq_inputs) }
    }

    fn is_valid(&self) -> bool {
        let mut iterator = self.input.to_iter();
        let mut possibilities: Vec<u64> = Vec::new();
        if let Some(num) = iterator.next() {
            possibilities.push(*num);
        }
        while let Some(num) = iterator.next() {
            let mut next_set = Vec::new();
            for i in possibilities {
                let plus = i + num;
                let mult = i * num;
                if plus <= self.outcome {
                    next_set.push(plus);
                }
                if mult <= self.outcome {
                    next_set.push(mult);
                }
            }
            possibilities = next_set;
        }
        if possibilities.contains(&self.outcome) {
            return true;
        } else {
            return false;
        }
    }

    fn is_valid_p2(&self) -> bool {
        let mut iterator = self.input.to_iter();
        let mut possibilities = Vec::new();
        if let Some(num) = iterator.next() {
            possibilities.push(*num);
        }
        while let Some(num) = iterator.next() {
            let mut next_set = Vec::new();
            for i in possibilities {
                let plus = i + num;
                let mult = i * num;
                let concat = concat_num(i, *num);
                if plus <= self.outcome {
                    next_set.push(plus);
                }
                if mult <= self.outcome {
                    next_set.push(mult);
                }
                if concat <= self.outcome {
                    next_set.push(concat);
                }
            }
            possibilities = next_set;
        }
        if possibilities.contains(&self.outcome) {
            return true;
        } else {
            return false;
        }
    }
}

impl EquationInput {
    fn to_iter(&self) -> std::slice::Iter<'_, u64> {
        self.0.iter()
    }
}

fn parser(input: Vec<String>) -> Vec<Equation> {
    let mut output = Vec::new();
    for line in input {
        output.push(Equation::new_from_input(line));
    }
    return output;
}

fn concat_num(first: u64, second: u64) -> u64 {
    let digits = (0..).take_while(|i| 10u64.pow(*i) <= second).count();
    first * (10u64.pow(digits as u32)) + second 
}