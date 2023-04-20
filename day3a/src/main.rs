use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("unable to read file");
    let mut sum = 0;
    for line in contents.lines() {
        let n = line.len();
        let rucksack: Vec<char> = line.chars().collect();
        let compartment1: HashSet<char> = HashSet::from_iter(rucksack[..n/2].to_vec());
        let compartment2: HashSet<char> = HashSet::from_iter(rucksack[n/2..].to_vec());
        let intersection = compartment1.intersection(&compartment2);
        for c in intersection {
            let item = *c as u32;
            if item <= 90 {
                sum += item - 38;
            } else {
                sum += item - 96;
            }
        }
    }
    println!("{}", sum);
}
