mod common;
use std::str::FromStr;

fn part1(codes: &[Seat]) -> u32 {
    codes.iter().map(Seat::id).max().expect("no seats")
}

fn part2(codes: &[Seat]) -> u32 {
    let mut ids: Vec<_> = codes.iter().map(Seat::id).collect();
    ids.sort_unstable();
    use std::convert::TryFrom as _;
    ids.windows(2)
        .flat_map(<&[u32; 2]>::try_from)
        .find_map(|&[i1, i2]| if i1 + 2 == i2 { Some(i1 + 1) } else { None })
        //.find_map(|&[i1, i2]| (i1 + 2 == i2).then(|| i1 + 1))
        .expect("id not found")
}

#[derive(Debug)]
struct Seat {
    row: u32,
    column: u32,
}

impl Seat {
    fn id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

fn binary_search<I, T>(
    mut lower: u32,
    mut upper: u32,
    down: T,
    up: T,
    iter: &mut I,
) -> Result<u32, String>
where
    I: Iterator<Item = T>,
    T: std::fmt::Display + Eq,
{
    assert!(lower < upper);
    while upper - lower >= 2 {
        let mid = lower + ((upper - lower) / 2);
        let dir = iter.next().ok_or_else(|| "not enough directions".to_string())?;
        if dir == down {
            upper = mid;
        } else if dir == up {
            lower = mid;
        } else {
            return Err(format!("invalid direction: {}", dir));
        }
    }
    Ok(lower)
}

impl FromStr for Seat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let row = binary_search(0, 128, 'F', 'B', &mut chars)?;
        let column = binary_search(0, 8, 'L', 'R', &mut chars)?;
        Ok(Seat { row, column })
    }
}

fn main() {
    let input = common::get_lines();
    let seats: Vec<_> = input
        .iter()
        .map(|s| s.parse::<Seat>().expect("could not parse seat"))
        .collect();
    println!("Part1: {}", part1(&seats));
    println!("Part2: {}", part2(&seats));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s1 = "FBFBBFFRLR";
        let s2 = "BFFFBBFRRR";
        let s3 = "FFFBBBFRRR";
        let s4 = "BBFFBBFRLL";

        let seat1 = s1.parse::<Seat>().unwrap();
        let seat2 = s2.parse::<Seat>().unwrap();
        let seat3 = s3.parse::<Seat>().unwrap();
        let seat4 = s4.parse::<Seat>().unwrap();

        assert_eq!(seat1.row, 44);
        assert_eq!(seat1.column, 5);
        assert_eq!(seat1.id(), 357);

        assert_eq!(seat2.row, 70);
        assert_eq!(seat2.column, 7);
        assert_eq!(seat2.id(), 567);

        assert_eq!(seat3.row, 14);
        assert_eq!(seat3.column, 7);
        assert_eq!(seat3.id(), 119);

        assert_eq!(seat4.row, 102);
        assert_eq!(seat4.column, 4);
        assert_eq!(seat4.id(), 820);
    }
}
