use ansi_term::{Colour, Style};
use std::collections::HashMap;
use std::iter;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::{thread, time};
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
        Screen {
            board: vec![vec![Tile::Empty; width]; height],
        }
    }
    #[allow(dead_code)]
    fn get(&self, x: usize, y: usize) -> Tile {
        self.board[y][x]
    }
    fn set(&mut self, x: usize, y: usize, t: Tile) {
        self.board[y][x] = t;
    }
    fn count(&self, t: Tile) -> usize {
        self.board
            .iter()
            .flat_map(|r| r.iter())
            .filter(|&&u| u == t)
            .count()
    }
    fn get_position_of(&self, t: Tile) -> Option<Point2D> {
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                if self.board[y][x] == t {
                    return Some(Point2D {
                        x: x as i64,
                        y: y as i64,
                    });
                }
            }
        }
        None
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

fn parse_tile(x: i64) -> Tile {
    match x {
        0 => Tile::Empty,
        1 => Tile::Wall,
        2 => Tile::Block,
        3 => Tile::Paddle,
        4 => Tile::Ball,
        _ => panic!("unknown tile"),
    }
}

fn construct_screen(output: &[i64]) -> Screen {
    let mut tiles: HashMap<Point2D, Tile> = HashMap::new();

    for c in output.chunks(3) {
        let p = Point2D { x: c[0], y: c[1] };
        let t = parse_tile(c[2]);
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

fn empty_run(code: &[i64]) -> Screen {
    let p = intcode::ProgramRunner::new(code);
    let output = p.run_with(&[]);
    construct_screen(&output)
}

fn outputter(or: Receiver<Option<i64>>, screen: Arc<Mutex<Screen>>, mut score: i64) {
    screen.lock().unwrap().print();
    println!("Score: {}", score);

    while let Ok(Some(x)) = or.recv() {
        let y = or.recv().unwrap().unwrap();
        let z = or.recv().unwrap().unwrap();
        if x == -1 && y == 0 {
            score = z;
        } else {
            let t = parse_tile(z);
            screen.lock().unwrap().set(x as usize, y as usize, t);
        }
        print!("{}[2J", 27 as char);
        screen.lock().unwrap().print();
        println!("Score: {}", score);
    }
}

fn inputter(is: Sender<i64>, screen: Arc<Mutex<Screen>>) {
    let mut inputs = Vec::new();
    loop {
        let update_time = time::Duration::from_millis(100);
        thread::sleep(update_time);
        let paddle;
        let ball;
        {
            let guard = screen.lock().unwrap();
            paddle = guard.get_position_of(Tile::Paddle);
            ball = guard.get_position_of(Tile::Ball);
        }
        let signal = match (ball, paddle) {
            (Some(pb), Some(pp)) => Some((pb.x - pp.x).signum()),
            _ => None,
        };
        if let Some(x) = signal {
            let r = is.send(x);
            if r.is_err() {
                println!("Set of inputs: {:?}", inputs);
                println!("Error while sending");
                return;
            }
            inputs.push(x);
        }
    }
}

#[allow(dead_code)]
fn play_game(code: &[i64]) {
    let mut with_coins = code.to_vec();
    with_coins[0] = 2;
    let intcode::ProgramRunner { program, io } = intcode::ProgramRunner::new(&with_coins);
    let intcode::ProgramIO { is, or } = io;

    let program_thread = std::thread::spawn(move || {
        program.run();
    });
    let mut initial_output = Vec::new();
    let score;

    loop {
        match or.recv() {
            Err(_) => {
                println!("Error receiving output");
                return;
            }
            Ok(None) => {
                println!("Game halted prematurely");
                return;
            }
            Ok(Some(x)) => {
                if x == -1 {
                    let _ = or.recv();
                    score = or.recv().unwrap().expect("expected score");
                    break;
                } else {
                    initial_output.push(x);
                }
            }
        }
    }

    let screen = construct_screen(&initial_output);

    let mutex = std::sync::Mutex::new(screen);
    let arc = std::sync::Arc::new(mutex);

    let arc_out = arc.clone();
    let output_thread = std::thread::spawn(move || {
        outputter(or, arc_out, score);
    });
    let arc_in = arc.clone();
    let input_thread = std::thread::spawn(move || {
        inputter(is, arc_in);
    });

    program_thread.join().expect("could not join thread");
    output_thread.join().expect("could not join thread");
    input_thread.join().expect("could not join thread");
}

fn winning() -> Vec<i64> {
    // winning sequence obtained by playing the game
    let compressed = vec![
        4, 23, 27, 7, 8, 28, 14, 7, 30, 3, 3, 4, 1, 1, 2, 9, 11, 7, 7, 1, 1, 22, 1, 2, 23, 23, 14,
        13, 13, 15, 1, 7, 1, 1, 2, 3, 1, 2, 3, 4, 6, 5, 24, 10, 2, 17, 19, 20, 1, 2, 19, 18, 17,
        17, 18, 2, 11, 11, 14, 15, 19, 11, 11, 3, 3, 12, 3, 28, 28, 17, 19, 16, 17, 31, 13, 13, 1,
        1, 14, 1, 3, 3, 16, 5, 2, 4, 15, 31, 14, 20, 37, 4, 4, 16, 16, 4, 4, 18, 18, 29, 22, 30,
        14, 14, 7, 7, 15, 3, 17, 29, 13, 13, 11, 11, 14, 14, 31, 20, 1, 2, 20, 20, 22, 32, 16, 16,
        10, 10, 16, 16, 14, 3, 5, 6, 27, 17, 17, 2, 2, 18, 1, 17, 18, 21, 5, 5, 21, 21, 31, 28, 3,
        6, 21, 21, 6, 6, 22, 1, 2, 23, 21, 21, 25, 25, 1, 1, 26, 26, 2, 2, 27, 27, 2, 2, 27, 27, 3,
        3, 28, 2, 1, 3, 4, 3, 3, 28, 3, 3, 29, 29, 6, 2, 4, 5, 8, 11, 31, 31, 1, 1, 32, 6, 7, 33,
        37, 18, 18, 14, 14, 19, 1, 2, 20, 7, 7, 21, 21, 37, 8, 8, 27, 27, 10, 10, 29, 29, 11, 11,
        30, 30, 4, 4, 31, 31, 5, 5, 5, 5, 22, 22, 6, 6, 31, 31, 34, 34, 30, 8, 15, 22, 2, 2, 4, 19,
        14, 14, 37, 22, 22, 37, 15, 15, 37, 37, 30, 30, 14, 14, 31, 31, 37, 37, 37, 23, 23, 37, 37,
        37, 32, 32, 37, 37, 37, 27, 1, 2, 3, 12, 32, 32, 16, 16, 20, 20, 37, 37, 32, 32, 37, 37,
        33, 33, 35, 35, 33, 33, 37, 37, 37, 3, 3, 5, 5, 37, 34, 34, 37, 37, 37, 37, 37, 37, 37, 3,
    ];
    let mut winning = vec![0, 0, 0];
    let mut direction = 1;
    for n in compressed {
        winning.extend(iter::repeat(direction).take(n));
        direction = -direction;
    }
    winning
}

fn winning_run(code: &[i64]) -> i64 {
    let mut with_coins = code.to_vec();
    with_coins[0] = 2;
    let p = intcode::ProgramRunner::new(&with_coins);
    let w = winning();
    let output = p.run_with(&w);
    *output.last().unwrap()
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
        let screen = empty_run(&program);
        screen.print();
        let result1 = screen.count(Tile::Block);
        println!("Part1: screen has {} block tiles", result1);

        //play_game(&program);

        let result2 = winning_run(&program);
        println!("Part2: winning run has score {}", result2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let program = vec![104, 1, 104, 2, 104, 3, 104, 6, 104, 5, 104, 4, 99];
        let screen = construct_screen(&program);
        let paddles = screen.count(Tile::Paddle);
        let balls = screen.count(Tile::Ball);
        assert_eq!(paddles, 1);
        assert_eq!(balls, 1);
        assert_eq!(screen.get(1, 2), Tile::Paddle);
        assert_eq!(screen.get(6, 5), Tile::Ball);
    }
}
