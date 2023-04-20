use std::fs;
use std::iter::FromIterator;
use std::collections::{VecDeque, HashSet};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut buffer: VecDeque<char> = VecDeque::new();
    for (pos, c) in input.chars().enumerate() {
        move_buffer(c, &mut buffer);
        if check_buffer(&buffer) {
            println!("{}", pos + 1);
            break
        }
    }
}

fn check_buffer(buffer: &VecDeque<char>) -> bool {
    let buffer_len = buffer.len();
    let set = HashSet::<&char>::from_iter(buffer);
    buffer_len == 14 && buffer_len == set.len()
}

fn move_buffer(c: char, buffer: &mut VecDeque<char>) {
    if buffer.len() == 14 {
        buffer.pop_front();
    }
    buffer.push_back(c);
}