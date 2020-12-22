mod common;

use std::collections::HashSet;

fn answers(answer: &str) -> HashSet<char> {
    answer.chars().collect::<HashSet<_>>()
}

fn answers_any(group: &[HashSet<char>]) -> HashSet<char> {
    group.iter().fold(HashSet::new(), |a, b| &a | b)
}

fn answers_all(group: &[HashSet<char>]) -> HashSet<char> {
    group.iter().fold(answers_any(group), |a, b| &a & b)
}

fn part1<U: AsRef<[HashSet<char>]>>(groups: &[U]) -> usize {
    groups.iter().map(|g| answers_any(g.as_ref()).len()).sum()
}

fn part2<U: AsRef<[HashSet<char>]>>(groups: &[U]) -> usize {
    groups.iter().map(|g| answers_all(g.as_ref()).len()).sum()
}

fn parse_input(input: &str) -> Vec<Vec<HashSet<char>>> {
    let mut groups = Vec::new();
    let mut cur_group = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            groups.push(cur_group);
            cur_group = Vec::new();
        } else {
            cur_group.push(answers(line));
        }
    }
    if !cur_group.is_empty() {
        groups.push(cur_group);
    }
    groups
}

fn main() {
    let input = common::get_content();
    let answers = parse_input(&input);
    println!("Part1: {}", part1(&answers));
    println!("Part2: {}", part2(&answers));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "abc\n\
        \n\
        a\n\
        b\n\
        c\n\
        \n\
        ab\n\
        ac\n\
        \n\
        a\n\
        a\n\
        a\n\
        a\n\
        \n\
        b";

    #[test]
    fn test_part1() {
        let answers = parse_input(TEST_INPUT);
        assert_eq!(part1(&answers), 11);
    }

    #[test]
    fn test_part2() {
        let answers = parse_input(TEST_INPUT);
        assert_eq!(part2(&answers), 6);
    }
}
