use std::fs;

fn main() {
    let mut monkeys = parse_monkeys("input");
    let cycle_length = calculate_cycle_length(&monkeys);
    for _ in 1..=10000 {
        round(&mut monkeys, cycle_length);
    }
    println!("== After round 10000  ==");
    for (j, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {} inspected items {} times.", j, monkey.inspected);
    }
    monkey_business(&mut monkeys);
}

type TestFn = Box<dyn Fn(u64)->usize>;
type OperationFn = Box<dyn Fn(u64)->Option<u64>>;

struct Test {
    divisor: u64,
    monkey1: u64,
    monkey2: u64,
}

struct Operation {
    symbol: char,
    num: Option<u64>,
}

struct Monkey {
    items: Vec<u64>,
    test: TestFn,
    operation: OperationFn,
    inspected: u64,
    divisor: u64,
}

impl Monkey {
    fn new(text: &str) -> Self {
        let mut lines = text.lines();
        lines.next();
        let mut line = lines.next().unwrap();
        let mut iter = line.split(":");
        iter.next();
        let items: Vec<u64> = iter.next().unwrap().split(", ").map(|x| x.trim().parse().unwrap()).collect(); 
        line = lines.next().unwrap();
        iter = line.split("Operation: new = old ");
        iter.next();
        let mut iter = iter.next().unwrap().split(" ");
        let symbol = iter.next().unwrap().chars().next().unwrap();
        let tmp = iter.next().unwrap();
        let num = match tmp {
            "old" => None,
            _ => Some(tmp.parse().unwrap()),
        };
        let operation = Operation { symbol: symbol, num:  num};  
  
        line = lines.next().unwrap();
        iter = line.split("by ");
        iter.next();
        let divisor: u64 = iter.next().unwrap().parse().unwrap();
        line = lines.next().unwrap();
        iter = line.split("If true: throw to monkey ");
        iter.next();
        let monkey1: u64 = iter.next().unwrap().parse().unwrap();
        line = lines.next().unwrap();
        iter = line.split("If false: throw to monkey ");
        iter.next();
        let monkey2: u64 = iter.next().unwrap().parse().unwrap();
        let test = Test { divisor: divisor, monkey1: monkey1, monkey2: monkey2 };

        let test = Box::new(move |x| match x % test.divisor == 0 {
            true => test.monkey1 as usize,
            false => test.monkey2 as usize,
        });

        let operation = Box::new(move |x| match operation.symbol {
            '*' => match operation.num {
                Some(n) => Some(x * n),
                None => Some(x * x), 
            }
            '+' => match operation.num {
                Some(n) => Some(x + n),
                None => Some(x + x), 
            }
            _ => None,
        });

        Self { test: test, operation: operation, items: items.into(), inspected: 0 , divisor: divisor }
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let input = fs::read_to_string(input).unwrap();
    let monkey_list: Vec<&str> = input.split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = Vec::new();
    for text in monkey_list.iter() {
        monkeys.push(Monkey::new(text));
    }
    monkeys
}

fn turn(i: usize, monkeys: &mut Vec<Monkey>, cycle_length: u64) {
    let items = monkeys[i].items.clone();
    for item in items {
        let mut worry = (monkeys[i].operation)(item).unwrap();
        worry = worry % cycle_length;
        let target = (monkeys[i].test)(worry);
        monkeys[target].items.push(worry);
        monkeys[i].inspected +=1;
    }
    monkeys[i].items.clear();
}

fn round(monkeys: &mut Vec<Monkey>, cycle_length: u64) {
    for i in 0..monkeys.len() {
        turn(i, monkeys, cycle_length);
    }
}

fn monkey_business(monkeys: &mut Vec<Monkey>) {
    monkeys.sort_by_key(|x| x.inspected);
    monkeys.reverse();
    println!("{}", monkeys[0].inspected * monkeys[1].inspected);
}

fn calculate_cycle_length(monkeys: &Vec<Monkey>) -> u64 {
    let mut cycle_length = 1;
    for monkey in monkeys.iter() {
        cycle_length *= monkey.divisor;
    }
    cycle_length
}