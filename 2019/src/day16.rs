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
        self.repeat_pos += 1;
        if self.repeat_pos == self.repeat {
            self.repeat_pos = 0;
            self.pos = (self.pos + 1) % PatternIterator::BASE.len();
        }
        Some(Self::BASE[self.pos])
    }
}

impl PatternIterator {
    const BASE: [i32; 4] = [0, 1, 0, -1];

    fn new(repeat: usize, start: usize) -> PatternIterator {
        assert!(repeat > 0);
        PatternIterator {
            repeat,
            repeat_pos: start % repeat,
            pos: (start / repeat) % Self::BASE.len(),
        }
    }
}

fn fft(n: &mut Vec<i32>, phases: usize, start: usize) {
    for _ in 0..phases {
        for j in start..n.len() {
            let pattern = PatternIterator::new(start + j + 1, start + j);
            let result = n[j..].iter().zip(pattern).map(|(x, y)| x * y).sum::<i32>();
            n[j] = result.abs() % 10;
        }
    }
}

fn fft2(n: &mut Vec<i32>, phases: usize, start: usize) {
    for _ in 0..phases {
        let mut sum = n.iter().skip(start).sum::<i32>();
        #[allow(clippy::needless_range_loop)]
        for j in start..n.len() {
            let result = sum.abs() % 10;
            sum -= n[j];
            n[j] = result;
        }
    }
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

fn sequence_to_number<'a>(n: impl Iterator<Item = &'a i32>) -> i32 {
    n.fold(0, |s, x| 10 * s + x)
}

fn part2(n: &[i32], phases: usize, digits: usize) -> String {
    let offset = sequence_to_number(n.iter().take(7)) as usize;
    let mut input: Vec<_> = n.iter().cycle().take(10000 * n.len()).cloned().collect();
    if input.len() / 2 < offset {
        // apply optimized version
        fft2(&mut input, phases, offset);
    } else {
        println!("Warning: can not apply optimized version of FFT");
        fft(&mut input, phases, offset);
    }
    result_to_str(input.iter().skip(offset).take(digits))
}

fn main() -> Result<(), &'static str> {
    let numbers: Vec<_> = common::get_lines()
        .into_iter()
        .map(|l| parse_input(&l))
        .collect::<Result<Vec<_>, _>>()?;

    for n in numbers {
        let mut input1 = n.clone();
        fft(&mut input1, 100, 0);
        let output1 = result_to_str(n.iter().take(8));
        println!("Part1: result after 100 phases of FFT: {}", output1);

        let output2 = part2(&n, 100, 8);
        println!("Part2: message is {}", output2);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_part1(input: &str, phases: usize, digits: usize, expected: &str) {
        let mut n = parse_input(input).unwrap();
        fft(&mut n, phases, 0);
        let output = result_to_str(n.iter().take(digits));
        assert_eq!(output, expected);
    }

    fn test_part2(input: &str, phases: usize, digits: usize, expected: &str) {
        let n = parse_input(input).unwrap();
        let output = part2(&n, phases, digits);
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

    #[test]
    fn test_example5() {
        test_part2("03036732577212944063491565474664", 100, 8, "84462026");
    }

    #[test]
    fn test_example6() {
        test_part2("02935109699940807407585447034323", 100, 8, "78725270");
    }

    #[test]
    fn test_example7() {
        test_part2("03081770884921959731165446850517", 100, 8, "53553731");
    }
}
