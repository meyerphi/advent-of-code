use std::collections::HashSet;
use std::env;
use std::io::BufRead;

#[allow(dead_code)]
fn main() {
    let filename = env::args().nth(1).expect("no filename given");

    let file = std::fs::File::open(filename).expect("could not open file");
    let buffer = std::io::BufReader::new(file);

    let numbers: Vec<i32> = buffer
        .lines()
        .map(|l| {
            l.expect("could not read line")
                .parse::<i32>()
                .expect("could not parse number")
        })
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
