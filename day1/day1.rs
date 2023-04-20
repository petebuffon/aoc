use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut sum = 0;
    let mut v = Vec::new();
    for line in contents.lines() {
        if line == "" {
            v.push(sum);
            sum = 0;
            continue;
        }
        sum += line.parse::<i32>().unwrap();
    }
    // println!("{:?}", v);
    // let mut max = 0;
    // for n in v.iter() {
    //     if n > &max {
    //         max = *n;
    //     }
    // }
    // println!("{}", max);
    v.sort();
    let first = v.pop().unwrap();
    let second = v.pop().unwrap();
    let third = v.pop().unwrap();
    println!("{}, {}, {}", first, second, third);
    println!("{}", first + second + third);
    Ok(())
}
