use std::convert::TryFrom;
use std::fmt;
mod common;

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl TryFrom<Point3D> for Point2D {
    type Error = &'static str;

    fn try_from(p: Point3D) -> Result<Self, Self::Error> {
        if p.z == 0 {
            Err("3d point is a point in infinity")
        } else if p.x % p.z != 0 || p.y % p.z != 0 {
            Err("3d point is a non-integer 2d point")
        } else {
            Ok(Point2D {
                x: p.x / p.z,
                y: p.y / p.z,
            })
        }
    }
}

impl From<Point2D> for Point3D {
    fn from(p: Point2D) -> Point3D {
        Point3D {
            x: p.x,
            y: p.y,
            z: 1,
        }
    }
}

impl From<Edge> for Point3D {
    fn from(e: Edge) -> Point3D {
        Point3D::from(e.p).cross(Point3D::from(e.q))
    }
}

impl Point3D {
    fn cross(self, other: Point3D) -> Point3D {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Point3D { x, y, z }
    }
    fn inner(self, other: Point3D) -> i64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Edge {
    p: Point2D,
    q: Point2D,
}

impl Edge {
    fn new(p: Point2D, q: Point2D) -> Edge {
        Edge { p, q }
    }

    // tests if point r is on the edge
    fn contains_point(&self, r: Point2D) -> bool {
        // test if point is on line
        if Point3D::from(*self).inner(Point3D::from(r)) != 0 {
            false
        }
        // test if point is on line segment
        else {
            let min_x = std::cmp::min(self.p.x, self.q.x);
            let max_x = std::cmp::max(self.p.x, self.q.x);
            let min_y = std::cmp::min(self.p.y, self.q.y);
            let max_y = std::cmp::max(self.p.y, self.q.y);
            (min_x <= r.x && r.x <= max_x) && (min_y <= r.y && r.y <= max_y)
        }
    }
}

fn can_detect(origin: Point2D, target: Point2D, asteroids: &[Point2D]) -> bool {
    let edge = Edge::new(origin, target);
    for &a in asteroids {
        if a != origin && a != target && edge.contains_point(a) {
            return false;
        }
    }
    true
}

fn count_detect(origin: Point2D, asteroids: &[Point2D]) -> usize {
    asteroids
        .iter()
        .filter(|&&target| origin != target && can_detect(origin, target, &asteroids))
        .count()
}

fn max_detect(asteroids: &[Point2D]) -> (Point2D, usize) {
    asteroids
        .iter()
        .map(|&origin| (origin, count_detect(origin, &asteroids)))
        .max_by_key(|(_, n)| *n)
        .unwrap()
}

fn parse_asteroids(input: &[Vec<char>]) -> Vec<Point2D> {
    let mut result = Vec::new();
    for (y, row) in input.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '#' {
                result.push(Point2D {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }
    result
}

fn split_input(s: &str) -> Vec<Vec<char>> {
    s.split('\n').map(|l| l.chars().collect()).collect()
}

fn main() {
    let input = common::get_content();
    let asteroids = parse_asteroids(&split_input(&input));

    let (p, result1) = max_detect(&asteroids);
    println!(
        "Part1: Maximum number of asteroids detected by {}: {}",
        p, result1
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = ".#..#\n\
                     .....\n\
                     #####\n\
                     ....#\n\
                     ...##";
        let asteroids = parse_asteroids(&split_input(&input));
        let (p, n) = max_detect(&asteroids);
        assert_eq!(p, Point2D { x: 3, y: 4 });
        assert_eq!(n, 8);
    }

    #[test]
    fn test_example2() {
        let input = "......#.#.\n\
                     #..#.#....\n\
                     ..#######.\n\
                     .#.#.###..\n\
                     .#..#.....\n\
                     ..#....#.#\n\
                     #..#....#.\n\
                     .##.#..###\n\
                     ##...#..#.\n\
                     .#....####";
        let asteroids = parse_asteroids(&split_input(&input));
        let (p, n) = max_detect(&asteroids);
        assert_eq!(p, Point2D { x: 5, y: 8 });
        assert_eq!(n, 33);
    }

    #[test]
    fn test_example3() {
        let input = "#.#...#.#.\n\
                     .###....#.\n\
                     .#....#...\n\
                     ##.#.#.#.#\n\
                     ....#.#.#.\n\
                     .##..###.#\n\
                     ..#...##..\n\
                     ..##....##\n\
                     ......#...\n\
                     .####.###.";
        let asteroids = parse_asteroids(&split_input(&input));
        let (p, n) = max_detect(&asteroids);
        assert_eq!(p, Point2D { x: 1, y: 2 });
        assert_eq!(n, 35);
    }

    #[test]
    fn test_example4() {
        let input = ".#..#..###\n\
                     ####.###.#\n\
                     ....###.#.\n\
                     ..###.##.#\n\
                     ##.##.#.#.\n\
                     ....###..#\n\
                     ..#.#..#.#\n\
                     #..#.#.###\n\
                     .##...##.#\n\
                     .....#.#..";
        let asteroids = parse_asteroids(&split_input(&input));
        let (p, n) = max_detect(&asteroids);
        assert_eq!(p, Point2D { x: 6, y: 3 });
        assert_eq!(n, 41);
    }

    #[test]
    fn test_example5() {
        let input = ".#..##.###...#######\n\
                     ##.############..##.\n\
                     .#.######.########.#\n\
                     .###.#######.####.#.\n\
                     #####.##.#.##.###.##\n\
                     ..#####..#.#########\n\
                     ####################\n\
                     #.####....###.#.#.##\n\
                     ##.#################\n\
                     #####.##.###..####..\n\
                     ..######..##.#######\n\
                     ####.##.####...##..#\n\
                     .#####..#.######.###\n\
                     ##...#.##########...\n\
                     #.##########.#######\n\
                     .####.#.###.###.#.##\n\
                     ....##.##.###..#####\n\
                     .#.#.###########.###\n\
                     #.#.#.#####.####.###\n\
                     ###.##.####.##.#..##";
        let asteroids = parse_asteroids(&split_input(&input));
        let (p, n) = max_detect(&asteroids);
        assert_eq!(p, Point2D { x: 11, y: 13 });
        assert_eq!(n, 210);
    }
}
