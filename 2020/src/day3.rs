mod common;
use std::str::FromStr;

fn test_slope(map: &Map, x: usize, y: usize) -> usize {
    let mut pos = Position::new(map);
    let mut trees = 0;
    while !pos.is_at_bottom() {
        pos.update(x, y);
        trees += pos.is_at_tree() as usize;
    }
    trees
}

fn part1(map: &Map) -> usize {
    test_slope(map, 3, 1)
}

fn part2(map: &Map) -> usize {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut result = 1;
    for &(x, y) in slopes.iter() {
        result *= test_slope(map, x, y);
    }
    result
}

#[derive(Debug)]
struct Row {
    row: Vec<bool>,
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    rows: Vec<Row>,
}

struct Position<'a> {
    x: usize,
    y: usize,
    map: &'a Map,
}

impl<'a> Position<'a> {
    fn new(map: &'a Map) -> Position<'a> {
        Position { x: 0, y: 0, map }
    }

    fn update(&mut self, x_shift: usize, y_shift: usize) {
        self.x = (self.x + x_shift) % self.map.width;
        self.y = (self.y + y_shift) % self.map.height;
    }

    fn is_at_bottom(&self) -> bool {
        self.y + 1 == self.map.height
    }

    fn is_at_tree(&self) -> bool {
        self.map.rows[self.y].row[self.x]
    }
}

impl FromStr for Row {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row: Vec<_> = s
            .chars()
            .filter(|&c| c == '#' || c == '.')
            .map(|c| c == '#')
            .collect();
        Ok(Row { row })
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|l| l.parse::<Row>())
            .collect::<Result<Vec<_>, _>>()?;
        if rows.is_empty() {
            return Err("no rows".to_string());
        }
        let width = rows[0].row.len();
        if width == 0 {
            return Err("first row empty".to_string());
        }
        if rows.iter().any(|r| r.row.len() != width) {
            return Err("rows of different width".to_string());
        }
        let height = rows.len();
        Ok(Map {
            width,
            height,
            rows,
        })
    }
}

fn main() {
    let input = common::get_content().parse::<Map>().expect("invalid input");
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Map {
        let input = "..##.......\n\
            #...#...#..\n\
            .#....#..#.\n\
            ..#.#...#.#\n\
            .#...##..#.\n\
            ..#.##.....\n\
            .#.#.#....#\n\
            .#........#\n\
            #.##...#...\n\
            #...##....#\n\
            .#..#...#.#";
        input.parse::<Map>().unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&test_input()), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&test_input()), 336);
    }
}
