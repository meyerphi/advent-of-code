use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;
mod common;

enum Direction {
    U,
    D,
    L,
    R,
}

struct DirectedVec {
    direction: Direction,
    length: i64,
}

struct DirectedPath {
    segment: Vec<DirectedVec>,
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            "L" => Ok(Direction::L),
            "R" => Ok(Direction::R),
            _ => Err("unknown direction"),
        }
    }
}

impl FromStr for DirectedVec {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err("cannot parse empty directed vec")
        } else {
            let direction = s[0..1].parse::<Direction>()?;
            let length = s[1..].parse::<i64>().map_err(|_| "cannot parse length")?;
            Ok(DirectedVec { direction, length })
        }
    }
}

impl FromStr for DirectedPath {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seg = s
            .split(',')
            .map(|d| d.parse::<DirectedVec>())
            .collect::<Result<Vec<_>, Self::Err>>()?;
        Ok(DirectedPath { segment: seg })
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
    fn zero() -> Point2D {
        Point2D { x: 0, y: 0 }
    }

    fn manhattan_norm(&self) -> i64 {
        self.x.abs() + self.y.abs()
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

impl Point3D {
    fn cross(&self, other: &Point3D) -> Point3D {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Point3D { x, y, z }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Edge {
    p: Point2D,
    q: Point2D,
}

#[derive(Debug)]
struct Path {
    path: Vec<Point2D>,
}

impl PartialEq for Path {
    fn eq(&self, other: &Path) -> bool {
        self.path == other.path
    }
}

impl From<&DirectedPath> for Path {
    fn from(dp: &DirectedPath) -> Path {
        let mut x = 0;
        let mut y = 0;
        let p = Point2D { x, y };
        let mut path = vec![p];
        for v in &dp.segment {
            let l = v.length;
            match v.direction {
                Direction::U => y += l,
                Direction::D => y -= l,
                Direction::L => x -= l,
                Direction::R => x += l,
            }
            path.push(Point2D { x, y });
        }
        Path { path }
    }
}

struct EdgeIterator<'a> {
    path: &'a Path,
    index: usize,
}

impl<'a> Iterator for EdgeIterator<'a> {
    type Item = Edge;

    fn next(&mut self) -> Option<Edge> {
        if self.index + 1 >= self.path.path.len() {
            None
        } else {
            let p = self.path.path[self.index];
            let q = self.path.path[self.index + 1];
            self.index += 1;
            Some(Edge { p, q })
        }
    }
}

impl FromStr for Path {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dp = s.parse::<DirectedPath>()?;
        Ok(Path::from(&dp))
    }
}

impl Edge {
    // tests if point r is on the edge, assuming it
    // it is on the line extended by the edge
    fn contains_point(&self, r: Point2D) -> bool {
        let min_x = std::cmp::min(self.p.x, self.q.x);
        let max_x = std::cmp::max(self.p.x, self.q.x);
        let min_y = std::cmp::min(self.p.y, self.q.y);
        let max_y = std::cmp::max(self.p.y, self.q.y);
        (min_x <= r.x && r.x <= max_x) && (min_y <= r.y && r.y <= max_y)
    }
    fn intersect(&self, other: &Edge) -> Option<Point2D> {
        let p1 = Point3D::from(self.p);
        let q1 = Point3D::from(self.q);
        let p2 = Point3D::from(other.p);
        let q2 = Point3D::from(other.q);
        // note: parallel edges currently yield no intersection,
        // even if there is a unique intersection point
        let r = Point2D::try_from(p1.cross(&q1).cross(&(p2.cross(&q2)))).ok()?;
        // test if point lies on both edges
        if self.contains_point(r) && other.contains_point(r) {
            Some(r)
        } else {
            None
        }
    }
    fn manhattan_length(&self) -> i64 {
        (self.p - self.q).manhattan_norm()
    }
}

impl Path {
    fn edge_iter(&self) -> EdgeIterator {
        EdgeIterator {
            path: self,
            index: 0,
        }
    }
    fn intersect(&self, other: &Path) -> Vec<Point2D> {
        let mut intersections = Vec::new();
        for e1 in self.edge_iter() {
            for e2 in other.edge_iter() {
                if let Some(p) = e1.intersect(&e2) {
                    intersections.push(p);
                }
            }
        }
        intersections
    }
    // insert all points in inter into the path
    // assumes that all of the points in inter are on some edge
    fn with_intersections(&self, inter: &[Point2D]) -> Path {
        let mut new_path = Vec::new();
        new_path.push(self.path[0]);
        for e in self.edge_iter() {
            let origin = e.p;
            let mut new_points = Vec::new();
            for &p in inter {
                if p != e.p && p != e.q && e.contains_point(p) {
                    new_points.push(p);
                }
            }
            new_points.sort_by_key(|&p| Edge { p: origin, q: p }.manhattan_length());
            for p in new_points {
                new_path.push(p);
            }
            new_path.push(e.q);
        }
        Path { path: new_path }
    }
    fn compute_distances(&self) -> HashMap<Point2D, i64> {
        let mut map = HashMap::new();
        let mut start_dist = 0;
        map.insert(self.path[0], start_dist);
        for e in self.edge_iter() {
            // note: do *not* take a shortcut if the start point
            // was already visited
            // let start_dist = *map.get(&e.p).unwrap();
            let end_dist = start_dist + e.manhattan_length();
            start_dist = end_dist;
            map.entry(e.q).or_insert(end_dist);
        }
        map
    }
}

fn non_trivial_intersections(p1: &Path, p2: &Path) -> Vec<Point2D> {
    p1.intersect(&p2)
        .into_iter()
        .filter(|&p| p != Point2D::zero())
        .collect()
}

fn part1(p1: &Path, p2: &Path) -> Option<i64> {
    non_trivial_intersections(p1, p2)
        .into_iter()
        .map(|p| p.manhattan_norm())
        .min()
}

// adds all self-intersections and intersections of p1 and p2
// to both paths and returns the new paths (p1', p2')
fn extend_with_intersections(p1: &Path, p2: &Path) -> (Path, Path) {
    let mut p1inter = p1.intersect(&p1);
    let mut p2inter = p1.intersect(&p1);
    let intersections = p1.intersect(&p2);
    p1inter.extend(intersections.iter().cloned());
    p2inter.extend(intersections.iter().cloned());
    let p1new = p1.with_intersections(&p1inter);
    let p2new = p2.with_intersections(&p2inter);
    (p1new, p2new)
}

fn distance_to_intersections(p1: &Path, p2: &Path) -> Vec<(Point2D, i64, i64)> {
    let inter = non_trivial_intersections(p1, p2);
    let (p1new, p2new) = extend_with_intersections(p1, p2);
    let dist1 = p1new.compute_distances();
    let dist2 = p2new.compute_distances();
    inter
        .into_iter()
        .map(|p| (p, *dist1.get(&p).unwrap(), *dist2.get(&p).unwrap()))
        .collect()
}

fn part2(p1: &Path, p2: &Path) -> Option<i64> {
    distance_to_intersections(&p1, &p2)
        .into_iter()
        .map(|(_, d1, d2)| d1 + d2)
        .min()
}

fn main() {
    let paths: Vec<_> = common::get_lines()
        .into_iter()
        .map(|l| l.parse::<Path>().expect("could not parse path"))
        .collect();
    assert_eq!(paths.len(), 2);
    let p1 = &paths[0];
    let p2 = &paths[1];

    let r1 = part1(&p1, &p2);
    if let Some(d) = r1 {
        println!("Part1: Minimal distance {}", d);
    } else {
        println!("No non-trivial intersection found");
    }

    let r2 = part2(&p1, &p2);
    if let Some(d) = r2 {
        println!("Part2: Minimal distance {}", d);
    } else {
        println!("No non-trivial intersection found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example0() -> (Path, Path) {
        let p1 = "R8,U5,L5,D3".parse::<Path>().unwrap();
        let p2 = "U7,R6,D4,L4".parse::<Path>().unwrap();
        (p1, p2)
    }
    fn example1() -> (Path, Path) {
        let p1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72"
            .parse::<Path>()
            .unwrap();
        let p2 = "U62,R66,U55,R34,D71,R55,D58,R83".parse::<Path>().unwrap();
        (p1, p2)
    }
    fn example2() -> (Path, Path) {
        let p1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
            .parse::<Path>()
            .unwrap();
        let p2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            .parse::<Path>()
            .unwrap();
        (p1, p2)
    }

    #[test]
    fn test_intersection() {
        let (p1, p2) = example0();
        let mut inter = p1.intersect(&p2);
        let mut expected = vec![
            Point2D { x: 0, y: 0 },
            Point2D { x: 3, y: 3 },
            Point2D { x: 6, y: 5 },
        ];
        inter.sort();
        expected.sort();
        assert_eq!(inter, expected);
    }

    #[test]
    fn test_intersection_parallel() {
        let p1 = "R5".parse::<Path>().unwrap();
        let p2 = "L5".parse::<Path>().unwrap();
        let inter = p1.intersect(&p2);
        assert_eq!(inter, vec![]);
    }

    #[test]
    fn test_part1_ex1() {
        let (p1, p2) = example1();
        let d = part1(&p1, &p2);
        assert_eq!(d, Some(159));
    }

    #[test]
    fn test_part1_ex2() {
        let (p1, p2) = example2();
        let d = part1(&p1, &p2);
        assert_eq!(d, Some(135));
    }

    #[test]
    fn test_extend_intersections() {
        let (p1, p2) = example0();
        let (p1new, p2new) = extend_with_intersections(&p1, &p2);
        let p1expected = Path {
            path: vec![
                Point2D { x: 0, y: 0 },
                Point2D { x: 8, y: 0 },
                Point2D { x: 8, y: 5 },
                Point2D { x: 6, y: 5 },
                Point2D { x: 3, y: 5 },
                Point2D { x: 3, y: 3 },
                Point2D { x: 3, y: 2 },
            ],
        };
        let p2expected = Path {
            path: vec![
                Point2D { x: 0, y: 0 },
                Point2D { x: 0, y: 7 },
                Point2D { x: 6, y: 7 },
                Point2D { x: 6, y: 5 },
                Point2D { x: 6, y: 3 },
                Point2D { x: 3, y: 3 },
                Point2D { x: 2, y: 3 },
            ],
        };
        assert_eq!(p1new, p1expected);
        assert_eq!(p2new, p2expected);
    }
    #[test]
    fn test_part2_ex0() {
        let (p1, p2) = example0();
        let mut r = distance_to_intersections(&p1, &p2);
        let mut expected = vec![
            (Point2D { x: 6, y: 5 }, 15, 15),
            (Point2D { x: 3, y: 3 }, 20, 20),
        ];
        r.sort();
        expected.sort();
        assert_eq!(r, expected);
    }
    #[test]
    fn test_part2_ex1() {
        let (p1, p2) = example1();
        let d = part2(&p1, &p2);
        assert_eq!(d, Some(610));
    }

    #[test]
    fn test_part2_ex2() {
        let (p1, p2) = example2();
        let d = part2(&p1, &p2);
        assert_eq!(d, Some(410));
    }
}
