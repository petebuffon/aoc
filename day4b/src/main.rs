use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let nums: Vec<u32> = line.split(|c| c == '-' || c == ',')
            .map(|c| c.parse().unwrap())
            .collect();
        let a1 = Assignment::new(nums[0], nums[1]);
        let a2 = Assignment::new(nums[2], nums[3]);
        if check(&a1, &a2) {
            sum += 1;
        } else if check(&a2, &a1) {
            sum += 1;
        }
    }
    println!("{}", sum);
}

#[derive(Debug)]
struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    fn new(start: u32, end: u32) -> Assignment {
        Assignment {
            start,
            end,
        }
    }
}
        
fn check(a1: &Assignment, a2: &Assignment) -> bool {
    a2.start <= a1.end && a2.end >= a1.start
}
