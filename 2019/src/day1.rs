#[path = "common.rs"]
mod common;

#[allow(dead_code)]
fn main() {
    let result: i32 = common::get_lines()
        .into_iter()
        .map(|l| l.parse::<i32>().expect("could not parse number"))
        .map(|n| n / 3 - 2)
        .sum();
    println!("Total fuel required: {}", result);
}
