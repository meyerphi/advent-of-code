use std::collections::HashMap;
use std::env;
use std::io::BufRead;

#[allow(dead_code)]
fn main() {
    let filename = env::args().nth(1).expect("no filename given");

    let file = std::fs::File::open(filename).expect("could not open file");
    let buffer = std::io::BufReader::new(file);

    let boxes: Vec<String> = buffer
        .lines()
        .map(|l| l.expect("could not read line"))
        .collect();
    let mut two = 0;
    let mut three = 0;
    for x in boxes {
        let mut letters = HashMap::with_capacity(x.len());

        for c in x.chars() {
            *letters.entry(c).or_insert(0) += 1;
        }

        two += letters.values().any(|&n| n == 2) as i32;
        three += letters.values().any(|&n| n == 3) as i32;
    }
    let result = two * three;
    println!("Checksum: {}", result);
}
