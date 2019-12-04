mod common;
use itertools::Itertools;

fn get_digits(n: i64) -> [i8; 6] {
    assert!(n >= 0);
    assert!(n <= 999_999);
    let mut x = n;
    let mut d = [0i8; 6];
    const EXP: [i64; 6] = [100_000, 10_000, 1000, 100, 10, 1];
    for i in 0..6usize {
        let m = EXP[i];
        d[i] = (x / m) as i8;
        x %= m;
    }
    d
}

#[allow(dead_code)]
fn check_criteria_part1_loop(n: i64) -> bool {
    let digits = get_digits(n);
    let mut last: i8 = -1;
    let mut has_double = false;
    for &d in &digits {
        if d < last {
            return false;
        }
        if d == last {
            has_double = true;
        }
        last = d;
    }
    has_double
}

fn check_criteria_part1_windows(n: i64) -> bool {
    let digits = get_digits(n);
    digits.windows(2).all(|w| w[0] <= w[1]) && digits.windows(2).any(|w| w[0] == w[1])
}

#[allow(dead_code)]
fn check_criteria_part2_iter(n: i64) -> bool {
    let digits = get_digits(n);
    let has_double = digits
        .iter()
        .group_by(|&d| d)
        .into_iter()
        .any(|(_, e)| e.count() == 2);
    let increasing = digits
        .iter()
        .fold((true, -1), |(inc, c), &d| (inc && c <= d, d))
        .0;
    has_double && increasing
}

#[allow(dead_code)]
fn check_criteria_part2_loop(n: i64) -> bool {
    let digits = get_digits(n);
    let mut last: i8 = -1;
    let mut digit_count = 1;
    let mut has_double = false;
    for &d in &digits {
        if d < last {
            return false;
        }
        if d == last {
            digit_count += 1;
        } else {
            if digit_count == 2 {
                has_double = true;
            }
            digit_count = 1;
        }
        last = d;
    }
    if digit_count == 2 {
        has_double = true;
    }
    has_double
}

fn count_passwords<F>(start: i64, end: i64, criterion: F) -> usize
where
    F: Fn(i64) -> bool,
{
    (start..=end).filter(|&n| criterion(n)).count()
}

fn main() {
    let input: Vec<Vec<_>> = common::get_lines()
        .into_iter()
        .map(|l| {
            l.split('-')
                .map(|i| i.parse::<i64>().expect("could not parse number"))
                .collect()
        })
        .collect();
    for range in input {
        assert!(range.len() == 2);
        let start = range[0];
        let end = range[1];

        let result1 = count_passwords(start, end, check_criteria_part1_windows);
        println!(
            "Part1: Number of matching passwords in range {}-{}: {}",
            start, end, result1
        );

        let result2 = count_passwords(start, end, check_criteria_part2_loop);
        println!(
            "Part2: Number of matching passwords in range {}-{}: {}",
            start, end, result2
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(get_digits(122_345), [1, 2, 2, 3, 4, 5]);
        assert_eq!(get_digits(111_123), [1, 1, 1, 1, 2, 3]);
        assert_eq!(get_digits(135_679), [1, 3, 5, 6, 7, 9]);
    }

    #[test]
    fn test_numbers_part1() {
        assert!(check_criteria_part1_loop(111_111));
        assert!(!check_criteria_part1_loop(223_450));
        assert!(!check_criteria_part1_loop(123_789));
    }
    #[test]
    fn test_numbers_part1_windows() {
        assert!(check_criteria_part1_windows(111_111));
        assert!(!check_criteria_part1_windows(223_450));
        assert!(!check_criteria_part1_windows(123_789));
    }

    #[test]
    fn test_numbers_part2() {
        assert!(check_criteria_part2_iter(112_233));
        assert!(!check_criteria_part2_iter(123_444));
        assert!(check_criteria_part2_iter(111_122));
    }

    #[test]
    fn test_numbers_part2_fast() {
        assert!(check_criteria_part2_loop(112_233));
        assert!(!check_criteria_part2_loop(123_444));
        assert!(check_criteria_part2_loop(111_122));
    }
}
