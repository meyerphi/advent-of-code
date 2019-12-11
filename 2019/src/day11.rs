use std::collections::HashMap;
use std::fmt;
use ansi_term::Colour::{Black, White};
mod common;
use common::intcode;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl Direction {
    fn turn_left(&mut self) {
        *self = match *self {
            Direction::U => Direction::L,
            Direction::D => Direction::R,
            Direction::L => Direction::D,
            Direction::R => Direction::U,
        };
    }
    fn turn_right(&mut self) {
        *self = match *self {
            Direction::U => Direction::R,
            Direction::D => Direction::L,
            Direction::L => Direction::U,
            Direction::R => Direction::D,
        };
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct Point2D {
    x: i64,
    y: i64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point2D {
    fn add_direction(&mut self, dir: Direction) {
        match dir {
            Direction::U => self.y -= 1,
            Direction::D => self.y += 1,
            Direction::L => self.x -= 1,
            Direction::R => self.x += 1,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Color {
    Black,
    White,
}

fn print_hull(hull: &HashMap<Point2D, Color>) {
    let min_x = hull.keys().map(|&p| p.x).min().unwrap_or(0);
    let max_x = hull.keys().map(|&p| p.x).max().unwrap_or(0);
    let min_y = hull.keys().map(|&p| p.y).min().unwrap_or(0);
    let max_y = hull.keys().map(|&p| p.y).max().unwrap_or(0);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Point2D { x, y };
            let color = hull.get(&point).copied().unwrap_or(Color::Black);
            let out = match color {
                Color::Black => White.on(Black).paint(" "),
                Color::White => Black.on(White).paint(" "),
            };
            print!("{}", out);
        }
        println!();
    }
}

fn run_robot(code: &[i64], initial: Option<Color>) -> HashMap<Point2D, Color> {
    let mut hull: HashMap<Point2D, Color> = HashMap::new();
    let mut position = Point2D { x: 0, y: 0 };
    let mut direction = Direction::U;

    if let Some(initial_color) = initial {
        hull.insert(position, initial_color);
    }

    let intcode::ProgramRunner { program, io } = intcode::ProgramRunner::new(code);
    let program_thread = std::thread::spawn(move || {
        program.run();
    });

    loop {
        let color = hull.get(&position).copied().unwrap_or(Color::Black);
        let input = match color {
            Color::Black => 0,
            Color::White => 1,
        };
        io.send_input_unchecked(input);
        if let Some(paint) = io.next_output() {
            let new_color = match paint {
                0 => Color::Black,
                1 => Color::White,
                _ => panic!("unknown color for painting"),
            };
            hull.insert(position, new_color);
            if let Some(turn) = io.next_output() {
                match turn {
                    0 => direction.turn_left(),
                    1 => direction.turn_right(),
                    _ => panic!("unknown turn direction"),
                }
                position.add_direction(direction);
            } else {
                break;
            }
        } else {
            break;
        }
    }
    program_thread.join().expect("could not join thread");

    hull
}

fn main() {
    let input: Vec<Vec<i64>> = common::get_lines()
        .into_iter()
        .map(|l| {
            l.split(',')
                .map(|i| i.parse::<i64>().expect("could not parse number"))
                .collect()
        })
        .collect();
    for program in input {
        let result1 = run_robot(&program, None);
        println!("Part1: panels painted on: {}", result1.len());

        let result2 = run_robot(&program, Some(Color::White));
        println!("Part2:");
        print_hull(&result2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let program = vec![
            3, 0, 104, 1, 104, 0, 3, 0, 104, 0, 104, 0, 3, 0, 104, 1, 104, 0, 104, 1, 104, 0, 3, 1,
            104, 0, 104, 1, 3, 2, 104, 1, 104, 0, 3, 2, 104, 1, 104, 0, 99,
        ];
        let result = run_robot(&program, None);
        assert_eq!(result.len(), 6);
    }
}
