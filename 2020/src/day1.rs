mod common;

use std::collections::HashSet;

fn part1(entries: &[i32]) -> i32 {
    let set: HashSet<_> = entries.iter().copied().collect();
    for e1 in entries {
        let e2 = 2020 - e1;
        if set.contains(&e2) {
            return e1 * e2;
        }
    }
    panic!("no matching entries found")
}

fn part2(entries: &[i32]) -> i32 {
    let set: HashSet<_> = entries.iter().copied().collect();
    for (i, e1) in entries.iter().enumerate() {
        for e2 in &entries[i + 1..] {
            let e3 = 2020 - e1 - e2;
            if set.contains(&e3) {
                return e1 * e2 * e3;
            }
        }
    }
    panic!("no matching entries found")
}

fn main() {
    let input: Vec<i32> = common::get_input();
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&[1721, 979, 366, 299, 675, 1456]), 514579);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&[1721, 979, 366, 299, 675, 1456]), 241861950);
    }
}
