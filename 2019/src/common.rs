use std::env;
use std::io::BufRead;

#[allow(dead_code)]
pub fn get_lines() -> Vec<String> {
    let filename = env::args().nth(1).expect("no filename given");

    let file = std::fs::File::open(filename).expect("could not open file");
    let buffer = std::io::BufReader::new(file);

    buffer
        .lines()
        .map(|l| l.expect("could not read line"))
        .collect()
}
