mod common;

fn get_digits(n: i64) -> [i8; 6] {
    let mut x = n;
    assert!(n >= 0);
    assert!(n <= 999_999);
    let d0 = (x / 100_000) as i8;
    x %= 100_000;
    let d1 = (x / 10_000) as i8;
    x %= 10_000;
    let d2 = (x / 1000) as i8;
    x %= 1000;
    let d3 = (x / 100) as i8;
    x %= 100;
    let d4 = (x / 10) as i8;
    x %= 10;
    let d5 = x as i8;
    [d0, d1, d2, d3, d4, d5]
}

fn check_criteria_part1(n: i64) -> bool {
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
        let result1 = count_passwords(start, end, check_criteria_part1);
        println!(
            "Part1: Number of matching passwords in range {}-{}: {}",
            start, end, result1
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
        assert!(check_criteria_part1(111_111));
        assert!(!check_criteria_part1(223_450));
        assert!(!check_criteria_part1(123_789));
    }
}
