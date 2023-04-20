use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("unable to read file");
    let mut sum = 0;
    let lines = contents.lines().collect::<Vec<&str>>();
    let chunks: Vec<&[&str]> = lines.chunks(3).collect();
    for chunk in chunks {
        let rucksack1: HashSet<char> = HashSet::from_iter(chunk[0].chars());
        let rucksack2: HashSet<char> = HashSet::from_iter(chunk[1].chars());
        let rucksack3: HashSet<char> = HashSet::from_iter(chunk[2].chars());
        let intersection_12: HashSet<char> = rucksack1.intersection(&rucksack2)
            .cloned()
            .collect();
        let intersection_123 = rucksack3.intersection(&intersection_12);
        for c in intersection_123 {
            let item = *c as u32;
            if item <= 90 {
                sum += item - 38;
            } else {
                sum += item - 96;
            }
        }
    }
    println!{"{}", sum};
}