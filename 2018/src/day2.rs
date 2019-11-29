use std::collections::HashMap;
#[path = "common.rs"]
mod common;

#[allow(dead_code)]
fn main() {
    let boxes: Vec<String> = common::get_lines();
    let mut two = 0;
    let mut three = 0;
    for x in &boxes {
        let mut letters = HashMap::with_capacity(x.len());

        for c in x.chars() {
            *letters.entry(c).or_insert(0) += 1;
        }

        two += letters.values().any(|&n| n == 2) as i32;
        three += letters.values().any(|&n| n == 3) as i32;
    }
    let result = two * three;
    println!("Checksum: {}", result);

    for (i, x) in boxes.iter().enumerate() {
        for y in boxes.iter().skip(i + 1) {
            let common_letters: String = x
                .chars()
                .zip(y.chars())
                .filter_map(|(c, d)| if c == d { Some(c) } else { None })
                .collect();
            if common_letters.len() + 1 == x.len() {
                println!("Common letters: {}", common_letters);
            }
        }
    }
}
