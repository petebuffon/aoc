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
    println!("{}", cpu.sum);
}

struct CPU {
    x: i32,
    cycle: i32,
    sum: i32,
}

impl CPU {
    fn new() -> CPU {
        CPU { x: 1, cycle: 0, sum: 0 }
    }

    fn execute_noop(&mut self) {
        self.cycle += 1;
        if self.cycle_check() {
            self.sum += self.cycle * self.x;
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

    fn cycle_check(&self) -> bool {
        self.cycle % 20 == 0 && self.cycle / 20 % 2 == 1
    }
}