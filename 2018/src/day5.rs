#[path = "common.rs"]
mod common;

fn react(polymer: &str) -> String {
    let mut s = String::from(polymer);
    loop {
        // add end of polymer character
        s.push('#');
        let mut t = String::new();
        let mut skip = false;
        for (c, d) in s.chars().zip(s.chars().skip(1)) {
            if skip {
                skip = false;
                continue;
            }
            if c != d && c.to_uppercase().next() == d.to_uppercase().next() {
                skip = true;
            }
            else {
                t.push(c)
            }
        }
        // remove end of polymer character again
        s.pop();
        if t == s {
            break;
        }
        else {
            s = t;
        }
    }
    s
}

#[allow(dead_code)]
fn main() {
    let input: Vec<String> = common::get_lines();

    for polymer in input {
        let result = react(&polymer);

        println!("Polymer after reaction has size: {}", result.len());
    }
}
