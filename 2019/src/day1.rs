#[path = "common.rs"]
mod common;

#[allow(dead_code)]
fn main() {
    let _: Vec<String> = common::get_lines();
    println!("Hello Advent of Code 2019!");
}
