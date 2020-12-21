use std::env;
use std::fs;
use std::io::BufRead;
use std::str::FromStr;

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

#[allow(dead_code)]
pub fn get_input<T: FromStr>() -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    get_lines()
        .into_iter()
        .map(|l| l.parse::<T>().expect("could not parse input"))
        .collect()
}
