#[path = "common.rs"]
mod common;

fn simple_fuel_for_mass(m: i32) -> i32 {
    std::cmp::max(m / 3 - 2, 0)
}

fn fuel_for_mass(m: i32) -> i32 {
    let mut mass = m;
    let mut fuel = 0;
    while mass > 0 {
        let f = simple_fuel_for_mass(mass);
        fuel += f;
        mass = f;
    }
    fuel
}

fn part1(masses: &[i32]) -> i32 {
    masses.iter().map(|&m| simple_fuel_for_mass(m)).sum()
}

fn part2(masses: &[i32]) -> i32 {
    masses.iter().map(|&m| fuel_for_mass(m)).sum()
}

#[allow(dead_code)]
fn main() {
    let input: Vec<i32> = common::get_lines()
        .into_iter()
        .map(|l| l.parse::<i32>().expect("could not parse number"))
        .collect();
    let result1 = part1(&input);
    let result2 = part2(&input);
    println!("Part1: Total fuel required: {}", result1);
    println!("Part2: Total fuel required: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&[12]), 2);
        assert_eq!(part1(&[14]), 2);
        assert_eq!(part1(&[1969]), 654);
        assert_eq!(part1(&[100_756]), 33583);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&[14]), 2);
        assert_eq!(part2(&[1969]), 966);
        assert_eq!(part2(&[100_756]), 50346);
    }
}
