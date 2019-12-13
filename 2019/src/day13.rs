use std::collections::HashMap;
use ansi_term::{Style, Colour};
mod common;
use common::intcode;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct Point2D {
    x: i64,
    y: i64,
}

struct Screen {
    board: Vec<Vec<Tile>>,
}

impl Screen {
    fn new(width: usize, height: usize) -> Screen {
        Screen { board: vec![vec![Tile::Empty; width]; height] }
    }
    fn get(&self, x: usize, y: usize) -> Tile {
        self.board[y][x]
    }
    fn set(&mut self, x: usize, y: usize, t: Tile) {
        self.board[y][x] = t;
    }
    fn count(&self, t: Tile) -> usize {
        self.board.iter().flat_map(|r| r.iter()).filter(|&&u| u == t).count()
    }
    fn print(&self) {
        let empty_style = Style::new().on(Colour::White);
        let block_style = Style::new().fg(Colour::Blue).on(Colour::White);
        let wall_style = Style::new().on(Colour::Black);
        let paddle_style = Style::new().fg(Colour::Red).on(Colour::White).bold();
        let ball_style = Style::new().fg(Colour::Red).on(Colour::White);
        for row in &self.board {
            for t in row {
                let out = match t {
                    Tile::Empty => empty_style.paint(" "),
                    Tile::Wall => wall_style.paint(" "),
                    Tile::Block => block_style.paint("\u{25A0}"),
                    Tile::Paddle => paddle_style.paint("\u{2015}"),
                    Tile::Ball => ball_style.paint("\u{25CF}"),
                };
                print!("{}", out);
            }
            println!();
        }
    }
}

fn construct_screen(code: &[i64]) -> Screen {
    let mut tiles: HashMap<Point2D, Tile> = HashMap::new();

    let p = intcode::ProgramRunner::new(code);
    let outputs = p.run_with(&[]);

    for c in outputs.chunks(3) {
        let p = Point2D { x: c[0], y: c[1] };
        let t = match c[2] {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("unknown tile"),
        };
        tiles.insert(p, t);
    }

    let min_x = tiles.keys().map(|&p| p.x).min().unwrap_or(0);
    let max_x = tiles.keys().map(|&p| p.x).max().unwrap_or(0);
    let min_y = tiles.keys().map(|&p| p.y).min().unwrap_or(0);
    let max_y = tiles.keys().map(|&p| p.y).max().unwrap_or(0);

    assert!(min_x >= 0);
    assert!(min_y >= 0);

    let mut screen = Screen::new((max_x + 1) as usize, (max_y + 1) as usize);

    for (p, &t) in tiles.iter() {
        screen.set(p.x as usize, p.y as usize, t);
    }
    screen
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
        let screen = construct_screen(&program);
        screen.print();
        let result1 = screen.count(Tile::Block);
        println!("Part1: screen has {} block tiles", result1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let program = vec![
            104, 1, 104, 2, 104, 3, 104, 6, 104, 5, 104, 4, 99
        ];
        let screen = construct_screen(&program);
        let paddles = screen.count(Tile::Paddle);
        let balls = screen.count(Tile::Ball);
        assert_eq!(paddles, 1);
        assert_eq!(balls, 1);
        assert_eq!(screen.get(1, 2), Tile::Paddle);
        assert_eq!(screen.get(6, 5), Tile::Ball);
    }
}
