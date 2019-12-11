use std::env;
use std::io::BufRead;
use std::fs;
pub mod intcode;

fn get_filename() -> String {
    env::args().nth(1).expect("no filename given")
}

#[allow(dead_code)]
pub fn get_content() -> String {
    let filename = get_filename();
    fs::read_to_string(filename).expect("could not read file")
}

#[allow(dead_code)]
pub fn get_lines() -> Vec<String> {
    let filename = get_filename();
    let file = std::fs::File::open(filename).expect("could not open file");
    let buffer = std::io::BufReader::new(file);

    buffer
        .lines()
        .map(|l| l.expect("could not read line"))
        .collect()
}
