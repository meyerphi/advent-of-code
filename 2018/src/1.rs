use std::collections::HashSet;
use std::env;
use std::io::BufRead;

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
    let result: i32 = std::iter::Sum::sum(numbers.iter());
    println!("Resulting frequency: {}", result);

    let mut visited = HashSet::new();
    let mut x = 0;
    visited.insert(x);
    let mut i = 0;
    loop {
        x += numbers[i];
        if !visited.insert(x) {
            println!("First frequency reached twice: {}", x);
            break;
        }
        i = (i + 1) % numbers.len();
    }
}
