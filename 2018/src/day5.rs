mod common;

fn react(polymer: &str) -> String {
    let mut s = String::from(polymer);
    loop {
        // add end of polymer character
        let mut t = String::new();
        let mut skip = false;
        for (c, d) in s
            .chars()
            .zip(s.chars().skip(1).chain(vec!['#'].into_iter()))
        {
            if skip {
                skip = false;
                continue;
            }
            if c != d && c.to_uppercase().next() == d.to_uppercase().next() {
                skip = true;
            } else {
                t.push(c)
            }
        }
        if t == s {
            break;
        } else {
            s = t;
        }
    }
    s
}

fn shortest_with_deletion(polymer: &str) -> String {
    let reacted = (b'A'..=b'Z').map(|b| {
        let c = b as char;
        let s: String = polymer
            .chars()
            .filter(|d| d.to_uppercase().next() != Some(c))
            .collect();
        react(&s)
    });
    reacted.min_by_key(|s| s.len()).unwrap()
}

fn main() {
    let input: Vec<String> = common::get_lines();

    for polymer in input {
        let result1 = react(&polymer);
        println!("Polymer after reaction has size: {}", result1.len());
        let result2 = shortest_with_deletion(&polymer);
        println!("Shortest polymer after deletion and reaction has size: {}", result2.len());
    }
}
