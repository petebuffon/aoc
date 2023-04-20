use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut cpu = CPU::new();
    for line in input.lines() {
        match line {
            "noop" => cpu.execute_noop(),
            _ => cpu.execute_addx(line),
        }
    }
    for row in cpu.ctr.iter() {
        println!("{}", row);
    }
}

struct CPU {
    x: i32,
    cycle: i32,
    screen: String,
    ctr: Vec<String>,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            x: 1,
            cycle: 0,
            screen: String::new(),
            ctr: Vec::new(),
        }
    }

    fn execute_noop(&mut self) {
        // println!("{} {} {}", self.cycle, self.x, self.sprite_check());
        match self.sprite_check() {
            true => self.screen.push('#'),
            false => self.screen.push('.'),
        }
        self.cycle += 1;
        if self.screen.len() == 40 {
            self.ctr.push(self.screen.clone());
            self.screen.clear();
            self.cycle = 0;
        }
    }

    fn execute_addx(&mut self, line: &str) {
        self.execute_noop();
        self.execute_noop();
        let mut iter = line.split(" ");
        iter.next();
        let instruction: i32 = iter.next().unwrap().parse().unwrap();
        self.x += instruction;
    }

    fn sprite_check(&self) -> bool {
        self.x <= self.cycle + 1 && self.x >= self.cycle - 1
    }
}