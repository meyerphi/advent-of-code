use std::cmp::Ordering;
use std::collections::HashSet;
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

impl std::ops::Sub for Point2D {
    type Output = Point2D;

    fn sub(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point2D {
    fn inner_product(self, other: Point2D) -> i64 {
        self.x * other.x + self.y * other.y
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
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

impl Point3D {
    fn inner_product(self, other: Point3D) -> i64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross_product(self, other: Point3D) -> Point3D {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Point3D { x, y, z }
    }
}

fn triple_product<P: Into<Point3D>, Q: Into<Point3D>, R: Into<Point3D>>(p: P, q: Q, r: R) -> i64 {
    p.into().cross_product(q.into()).inner_product(r.into())
}

// relative length of (p, r) in terms of vector (p, q)
fn rel_length(p: Point2D, q: Point2D, r: Point2D) -> i64 {
    (q - p).inner_product(r - p)
}

// tests if point r is on the line segment (p, q)
fn point_on_semgent(p: Point2D, q: Point2D, r: Point2D) -> bool {
    // test if point is on line
    triple_product(p, q, r) == 0 && {
        // test if point is on line segment
        let min_x = std::cmp::min(p.x, q.x);
        let max_x = std::cmp::max(p.x, q.x);
        let min_y = std::cmp::min(p.y, q.y);
        let max_y = std::cmp::max(p.y, q.y);
        (min_x <= r.x && r.x <= max_x) && (min_y <= r.y && r.y <= max_y)
    }
}

// compares angle (p, q, a) with angle (p, q, b)
// if angle is same, compare by distance to p
fn compare_by_angle(p: Point2D, q: Point2D, a: Point2D, b: Point2D) -> Ordering {
    assert!(a != p);
    assert!(b != p);
    if a == b {
        return Ordering::Equal;
    }
    let pqa = triple_product(p, q, a);
    let pqb = triple_product(p, q, b);

    if pqa == 0 && pqb == 0 {
        // a and b both on line (p, q)
        let da = rel_length(p, q, a);
        let db = rel_length(p, q, b);
        if da > 0 && db < 0 {
            // a in direction of (p, q), but b not
            Ordering::Less
        } else if db > 0 && da < 0 {
            // b in direction of (p, q), but a not
            Ordering::Greater
        } else {
            // compare by distance
            da.abs().cmp(&db.abs())
        }
    } else if pqa < 0 && pqb >= 0 {
        // a on left side and b on right side of (p, q)
        Ordering::Greater
    } else if pqa >= 0 && pqb < 0 {
        // a on right side and b on left side of (p, q)
        Ordering::Less
    } else {
        // a and b on same side of (p, q)
        let pba = triple_product(p, b, a);
        if pba < 0 {
            Ordering::Less
        } else if pba > 0 {
            Ordering::Greater
        } else {
            // a on edge (p, b)
            let da = rel_length(p, b, a);
            let db = rel_length(p, b, b);
            // compare by distance
            da.abs().cmp(&db.abs())
        }
    }
}

fn can_detect(origin: Point2D, target: Point2D, asteroids: &[Point2D]) -> bool {
    asteroids
        .iter()
        .all(|&a| a == origin || a == target || !point_on_semgent(origin, target, a))
}

fn find_detect(origin: Point2D, asteroids: &[Point2D]) -> Vec<Point2D> {
    asteroids
        .iter()
        .filter(|&&target| origin != target && can_detect(origin, target, &asteroids))
        .copied()
        .collect()
}

fn max_detect(asteroids: &[Point2D]) -> (Point2D, usize) {
    asteroids
        .iter()
        .map(|&origin| (origin, find_detect(origin, &asteroids).len()))
        .max_by_key(|(_, n)| *n)
        .unwrap()
}

fn vaporize_once(origin: Point2D, asteroids: &[Point2D]) -> Vec<Point2D> {
    let up = Point2D {
        x: origin.x,
        y: origin.y - 1,
    };
    let mut detectable = find_detect(origin, asteroids);
    // sort by angle (i.e. time hit by laser)
    detectable.sort_by(|&a, &b| compare_by_angle(origin, up, a, b));
    detectable
}

fn vaporize_all(origin: Point2D, asteroids: &[Point2D]) -> Vec<Point2D> {
    let mut remaining: HashSet<Point2D> = asteroids.iter().copied().collect();
    let mut destroyed = Vec::with_capacity(asteroids.len());
    loop {
        let current: Vec<_> = remaining.iter().copied().collect();
        let vaporized = vaporize_once(origin, &current);
        if vaporized.is_empty() {
            break;
        } else {
            for p in &vaporized {
                remaining.remove(p);
            }
            destroyed.extend(vaporized.into_iter());
        }
    }
    destroyed
}

fn parse_input(input: &str) -> Vec<Point2D> {
    let vec: Vec<Vec<_>> = input.split('\n').map(|l| l.chars().collect()).collect();
    let mut result = Vec::new();
    for (y, row) in vec.iter().enumerate() {
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

fn main() {
    let input = common::get_content();
    let asteroids = parse_input(&input);

    let (p, result1) = max_detect(&asteroids);
    println!(
        "Part1: Maximum number of asteroids detected by {}: {}",
        p, result1
    );

    let vaporized = vaporize_all(p, &asteroids);
    let q = vaporized[199];
    let result2 = 100 * q.x + q.y;
    println!("Part2: 200th vaporized asteroid: {} => {}", q, result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_LARGE: &str = ".#..##.###...#######\n\
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

    #[test]
    fn test_example1() {
        let input = ".#..#\n\
                     .....\n\
                     #####\n\
                     ....#\n\
                     ...##";
        let asteroids = parse_input(&input);
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
        let asteroids = parse_input(&input);
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
        let asteroids = parse_input(&input);
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
        let asteroids = parse_input(&input);
        let (p, n) = max_detect(&asteroids);
        assert_eq!(p, Point2D { x: 6, y: 3 });
        assert_eq!(n, 41);
    }

    #[test]
    fn test_example5() {
        let asteroids = parse_input(&EXAMPLE_LARGE);

        let (p, n) = max_detect(&asteroids);
        assert_eq!(p, Point2D { x: 11, y: 13 });
        assert_eq!(n, 210);
    }

    #[test]
    fn test_vaporize() {
        let input = ".#....#####...#..\n\
                     ##...##.#####..##\n\
                     ##...#...#.#####.\n\
                     ..#.....X...###..\n\
                     ..#.#.....#....##";
        let asteroids = parse_input(&input);
        let station = Point2D { x: 8, y: 3 };
        let vaporized = vaporize_all(station, &asteroids);
        println!("Vap: {:?}", vaporized);
        let expected = vec![
            Point2D { x: 8, y: 1 },
            Point2D { x: 9, y: 0 },
            Point2D { x: 9, y: 1 },
            Point2D { x: 10, y: 0 },
            Point2D { x: 9, y: 2 },
            Point2D { x: 11, y: 1 },
            Point2D { x: 12, y: 1 },
            Point2D { x: 11, y: 2 },
            Point2D { x: 15, y: 1 },
            Point2D { x: 12, y: 2 },
            Point2D { x: 13, y: 2 },
            Point2D { x: 14, y: 2 },
            Point2D { x: 15, y: 2 },
            Point2D { x: 12, y: 3 },
            Point2D { x: 16, y: 4 },
            Point2D { x: 15, y: 4 },
            Point2D { x: 10, y: 4 },
            Point2D { x: 4, y: 4 },
            Point2D { x: 2, y: 4 },
            Point2D { x: 2, y: 3 },
            Point2D { x: 0, y: 2 },
            Point2D { x: 1, y: 2 },
            Point2D { x: 0, y: 1 },
            Point2D { x: 1, y: 1 },
            Point2D { x: 5, y: 2 },
            Point2D { x: 1, y: 0 },
            Point2D { x: 5, y: 1 },
            Point2D { x: 6, y: 1 },
            Point2D { x: 6, y: 0 },
            Point2D { x: 7, y: 0 },
            Point2D { x: 8, y: 0 },
            Point2D { x: 10, y: 1 },
            Point2D { x: 14, y: 0 },
            Point2D { x: 16, y: 1 },
            Point2D { x: 13, y: 3 },
            Point2D { x: 14, y: 3 },
        ];
        assert_eq!(vaporized, expected);
    }

    #[test]
    fn test_vaporize_large() {
        let asteroids = parse_input(&EXAMPLE_LARGE);
        let station = Point2D { x: 11, y: 13 };
        let vaporized = vaporize_all(station, &asteroids);
        assert_eq!(vaporized.len(), 299);
        assert_eq!(vaporized[0], Point2D { x: 11, y: 12 });
        assert_eq!(vaporized[1], Point2D { x: 12, y: 1 });
        assert_eq!(vaporized[2], Point2D { x: 12, y: 2 });
        assert_eq!(vaporized[9], Point2D { x: 12, y: 8 });
        assert_eq!(vaporized[19], Point2D { x: 16, y: 0 });
        assert_eq!(vaporized[49], Point2D { x: 16, y: 9 });
        assert_eq!(vaporized[99], Point2D { x: 10, y: 16 });
        assert_eq!(vaporized[199], Point2D { x: 8, y: 2 });
        assert_eq!(vaporized[200], Point2D { x: 10, y: 9 });
        assert_eq!(vaporized[298], Point2D { x: 11, y: 1 });
    }
}
