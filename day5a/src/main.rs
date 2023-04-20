use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut iter = input.split("\n\n");
    let mut crates_input = iter.next().unwrap().lines().rev();

    let mut crates: HashMap<usize, Vec<_>> = HashMap::new();
    for i in 1..10 {
        crates.insert(i, Vec::<char>::new());
    }

    crates_input.next();
    for line in crates_input {
        let stack: Vec<char> = line[1..]
            .chars()
            .step_by(4)
            .collect();
        for (i, &c) in stack.iter().enumerate() {
            if c != ' ' {
                crates.get_mut(&(i+1)).unwrap().push(c);
            }
        }
    }

    let inst_input = iter.next().unwrap().lines();
    for line in inst_input {
        let instruction = Instruction::new(&line);
        for _ in 1..instruction.num + 1{
            let tmp = crates.get_mut(&instruction.start).unwrap().pop().unwrap();
            crates.get_mut(&instruction.end).unwrap().push(tmp);
        }
    }
    let mut answer = Vec::new();
    for key in 1..10 {
        answer.push(crates.get_mut(&key).unwrap().pop().unwrap());
    }
    println!("{}", answer.iter().cloned().collect::<String>())
}

struct Instruction {
    num: usize,
    start: usize,
    end: usize,
}

impl Instruction {
    fn new(input: &str) -> Instruction {
        let mut iter = input.split(" ");
        iter.next();
        let mut iter = iter.step_by(2);
        let num = iter.next().unwrap().parse().unwrap();
        let start = iter.next().unwrap().parse().unwrap();
        let end = iter.next().unwrap().parse().unwrap();

        Instruction { num, start, end }
    }
}