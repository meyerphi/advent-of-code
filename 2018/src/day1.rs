use std::collections::HashSet;
#[path = "common.rs"]
mod common;

#[allow(dead_code)]
fn main() {
    let numbers: Vec<i32> = common::get_lines()
        .into_iter()
        .map(|l| l.parse::<i32>().expect("could not parse number"))
        .collect();
    let result: i32 = numbers.iter().sum();
    println!("Resulting frequency: {}", result);

    let mut visited = HashSet::new();
    let mut x = 0;
    visited.insert(x);
    for n in numbers.iter().cycle() {
        x += n;
        if !visited.insert(x) {
            break;
        }
    }
    println!("First frequency reached twice: {}", x);
}
