use itertools::Itertools;
mod common;

struct PatternIterator {
    repeat: usize,
    repeat_pos: usize,
    pos: usize,
}

impl Iterator for PatternIterator {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        const BASE: [i32; 4] = [0, 1, 0, -1];
        self.repeat_pos += 1;
        if self.repeat_pos == self.repeat {
            self.repeat_pos = 0;
            self.pos = (self.pos + 1) % BASE.len();
        }
        Some(BASE[self.pos])
    }
}

impl PatternIterator {
    fn new(repeat: usize) -> PatternIterator {
        assert!(repeat > 0);
        PatternIterator { repeat, repeat_pos: 0, pos: 0 }
    }
}

fn fft(number: &[i32], phases: usize) -> Vec<i32> {
    let mut n = number.to_vec();
    for _ in 0..phases {
        let m = n.clone();
        for (j, x) in n.iter_mut().enumerate() {
            let pattern = PatternIterator::new(j + 1);
            let result = m.iter().zip(pattern).map(|(x, y)| x*y).sum::<i32>().abs() % 10;
            //fold(0, |s, x| (s + x + 10) % 10);
            *x = result;
        }
    }
    n
}

fn parse_input(s: &str) -> Result<Vec<i32>, &'static str> {
    s.chars()
        .map(|c| {
            c.to_digit(10)
                .map(|x| x as i32)
                .ok_or("could not parse digit")
        })
        .collect()
}

fn result_to_str<'a>(n: impl Iterator<Item = &'a i32>) -> String {
    n.map(|x| x.to_string()).join("")
}

fn main() -> Result<(), &'static str> {
    let numbers: Vec<_> = common::get_lines()
        .into_iter()
        .map(|l| parse_input(&l))
        .collect::<Result<Vec<_>, _>>()?;

    for n in numbers {
        let result1 = fft(&n, 100);
        let output = result_to_str(result1.iter().take(8));
        println!("Part1: result after 100 phases of FFT: {}", output);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_part1(input: &str, phases: usize, digits: usize, expected: &str) {
        let n = parse_input(input).unwrap();
        let result = fft(&n, phases);
        let output = result_to_str(result.iter().take(digits));
        assert_eq!(output, expected);
    }

    #[test]
    fn test_example1() {
        test_part1("12345678", 4, 8, "01029498");
    }

    #[test]
    fn test_example2() {
        test_part1("80871224585914546619083218645595", 100, 8, "24176176");
    }

    #[test]
    fn test_example3() {
        test_part1("19617804207202209144916044189917", 100, 8, "73745418");
    }

    #[test]
    fn test_example4() {
        test_part1("69317163492948606335995924319873", 100, 8, "52432133");
    }
}
