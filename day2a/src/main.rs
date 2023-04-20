use std::fs;

fn main() {
    let input = fs::read_to_string("input")
        .expect("File read error.");
    let mut score = 0;
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        if second == "X" {
            score += 0;
            if first == "A" {
                score += 3;
            } else if first == "B" {
                score += 1;
            } else if first == "C" {
                score += 2;
            }
        } else if second == "Y" {
            score += 3;
            if first == "A" {
                score += 1;
            } else if first == "B" {
                score += 2;
            } else if first == "C" {
                score += 3;
            }
        } else if second == "Z" {
            score += 6;
            if first == "A" {
                score += 2;
            } else if first == "B" {
                score += 3;
            } else if first == "C" {
                score += 1;
            }
        }
    }
    println!("{score}");
}