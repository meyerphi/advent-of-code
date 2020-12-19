mod common;
use std::str::FromStr;

fn valid1(input: &Input) -> bool {
    let count = input
        .password
        .chars()
        .filter(|&c| c == input.letter)
        .count();
    input.lower <= count && count <= input.upper
}

fn valid2(input: &Input) -> bool {
    let mut chars = input.password.chars();
    let c1 = chars.nth(input.lower - 1).unwrap();
    let c2 = chars.nth(input.upper - input.lower - 1).unwrap();
    (input.letter == c1) ^ (input.letter == c2)
}

fn part1(entries: &[Input]) -> usize {
    entries.iter().filter(|i| valid1(i)).count()
}

fn part2(entries: &[Input]) -> usize {
    entries.iter().filter(|i| valid2(i)).count()
}

struct Input {
    lower: usize,
    upper: usize,
    letter: char,
    password: String,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split([' ', ':', '-'].as_ref());
        let lower = split
            .next()
            .ok_or("missing lower index")?
            .parse::<usize>()
            .map_err(|err| err.to_string())?;
        let upper = split
            .next()
            .ok_or("missing upper index")?
            .parse::<usize>()
            .map_err(|err| err.to_string())?;
        let letter = split
            .next()
            .ok_or("missing character")?
            .chars()
            .next()
            .ok_or("no character")?;
        split.next().ok_or("missing colon")?;
        let password = split.next().ok_or("missing password")?.to_string();
        Ok(Input {
            lower,
            upper,
            letter,
            password,
        })
    }
}

fn main() {
    let input: Vec<Input> = common::get_input();
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Vec<Input> {
        let input = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        input.iter().map(|l| l.parse::<Input>().unwrap()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&test_input()), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&test_input()), 1);
    }
}
