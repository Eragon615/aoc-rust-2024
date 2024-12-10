use crate::Application;

impl Application {
    pub fn day9(self) {
        if self.args.part == 1 {
            self.d9p1();
        } else {
            self.d9p2();
        }
    }

    fn d9p1(self) {
        let mut answer = 0;
        let mut diskmap = Diskmap::new_from_input(self.input);
        diskmap.defrag();
        println!("{answer}");
    }

    fn d9p2(self) {
        let mut answer = 0;

        println!("{answer}");
    }
}

struct Diskmap {
    pos: usize,
    disk: Vec<Block>
}

#[derive(Debug,Clone, Copy)]
enum Block {
    Filespace(Filespace),
    Freespace(Freespace)
}


#[derive(Debug,Clone, Copy)]
struct Filespace {
    id_num: usize,
    length: usize
}


#[derive(Debug,Clone, Copy)]
struct Freespace {
    length: usize
}

impl Diskmap {
    fn new_from_input(input: Vec<String>) -> Self {
        let mut disk = Vec::new();
        let mut id_num = 0;
        for line in input {
            for (idx, num) in line.chars().enumerate() {
                if idx % 2 == 0 {
                    disk.push(Block::Filespace(Filespace{id_num, length: num.to_digit(10).expect("A letter snuck into the puzzle input.") as usize }));
                    id_num += 1;
                } else {
                    disk.push(Block::Freespace(Freespace { length: num.to_digit(10).expect("A letter snuck into the puzzle input.") as usize }));
                }
            }
        }
        return Self{pos: 0, disk};
    }

    fn defrag(&mut self) {
        let mut remainder: Option<Filespace> = None;
        let mut current_file: Filespace;
        for block in self.disk.iter() {
            if let Block::Freespace(free) = block {
                if let Some(leftover) = &remainder {
                    current_file = *leftover;
                } else {
                    while let Some(last_block) = self.disk.last_mut() {
                        if let Block::Freespace(_) = last_block {

                        }
                    }
                }
            }
        }
    }

}