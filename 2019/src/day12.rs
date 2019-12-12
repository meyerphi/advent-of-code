use num::integer::lcm;
use regex::Regex;
use std::fmt;
use std::str::FromStr;
mod common;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl std::ops::Add for Point3D {
    type Output = Point3D;

    fn add(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::AddAssign for Point3D {
    fn add_assign(&mut self, other: Point3D) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::Sub for Point3D {
    type Output = Point3D;

    fn sub(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::SubAssign for Point3D {
    fn sub_assign(&mut self, other: Point3D) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl FromStr for Point3D {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new("<x=(?P<x>[-0-9]*), *y=(?P<y>[-0-9]*), *z=(?P<z>[-0-9]*)>").unwrap();
        let caps = re.captures(s).ok_or("could not parse 3d point")?;
        let x = caps["x"]
            .parse::<i64>()
            .map_err(|_| "could not parse x coordinate")?;
        let y = caps["y"]
            .parse::<i64>()
            .map_err(|_| "could not parse y coordinate")?;
        let z = caps["z"]
            .parse::<i64>()
            .map_err(|_| "could not parse z coordinate")?;
        Ok(Point3D { x, y, z })
    }
}

impl Point3D {
    fn zero() -> Point3D {
        Point3D { x: 0, y: 0, z: 0 }
    }
    fn l1_norm(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
    fn signum(&self) -> Point3D {
        Point3D {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Moon {
    position: Point3D,
    velocity: Point3D,
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<pos={}, vel={}>", self.position, self.velocity)
    }
}

impl Moon {
    fn new(position: Point3D) -> Moon {
        Moon {
            position,
            velocity: Point3D::zero(),
        }
    }

    fn add_velocity_from(&mut self, other: &Moon) {
        self.velocity += (other.position - self.position).signum();
    }

    fn apply_velocity(&mut self) {
        self.position += self.velocity;
    }
}

fn parse_moons(input: Vec<String>) -> Vec<Moon> {
    input
        .into_iter()
        .map(|l| l.parse::<Point3D>().expect("could not parse moon"))
        .map(Moon::new)
        .collect()
}

fn change_velocity(moons: &mut Vec<Moon>) {
    let mcopy = moons.to_vec();
    for m in moons {
        for other in &mcopy {
            m.add_velocity_from(other);
        }
    }
}

fn apply_velocity(moons: &mut Vec<Moon>) {
    for m in moons {
        m.apply_velocity();
    }
}

fn simulate_moons(init: &[Moon], steps: usize) -> Vec<Moon> {
    let mut moons = init.to_vec();
    for _ in 0..steps {
        change_velocity(&mut moons);
        apply_velocity(&mut moons);
    }
    moons
}

fn find_repeat(init: &[Moon]) -> usize {
    let mut moons = init.to_vec();
    let mut repeat_x: Option<usize> = None;
    let mut repeat_y: Option<usize> = None;
    let mut repeat_z: Option<usize> = None;
    for s in 1.. {
        change_velocity(&mut moons);
        apply_velocity(&mut moons);

        let (rx, ry, rz): (bool, bool, bool) = init
            .iter()
            .zip(moons.iter())
            .map(|(&m1, &m2)| {
                (
                    m1.position.x == m2.position.x && m1.velocity.x == m2.velocity.x,
                    m1.position.y == m2.position.y && m1.velocity.y == m2.velocity.y,
                    m1.position.z == m2.position.z && m1.velocity.z == m2.velocity.z,
                )
            })
            .fold((true, true, true), |(rx, ry, rz), (x, y, z)| {
                (rx && x, ry && y, rz && z)
            });
        if repeat_x == None && rx {
            repeat_x = Some(s);
        }
        if repeat_y == None && ry {
            repeat_y = Some(s);
        }
        if repeat_z == None && rz {
            repeat_z = Some(s);
        }
        if let (Some(x), Some(y), Some(z)) = (repeat_x, repeat_y, repeat_z) {
            return lcm(lcm(x, y), z);
        }
    }
    unreachable!()
}

fn total_energy(moons: &[Moon]) -> i64 {
    moons
        .iter()
        .map(|m| m.position.l1_norm() * m.velocity.l1_norm())
        .sum()
}

fn main() {
    let moons = parse_moons(common::get_lines());

    let result_moons = simulate_moons(&moons, 1000);
    let result1 = total_energy(&result_moons);
    println!("Part1: total energy after 100 steps is {}", result1);
    let result2 = find_repeat(&moons);
    println!("Part2: universe repeats after {} steps", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = vec![
            "<x=-1, y=0, z=2>",
            "<x=2, y=-10, z=-7>",
            "<x=4, y=-8, z=8>",
            "<x=3, y=5, z=-1>",
        ];
        let init = parse_moons(input.iter().map(|s| s.to_string()).collect());
        let moons = simulate_moons(&init, 10);
        let result = total_energy(&moons);
        assert_eq!(result, 179);
        let repeat_num = find_repeat(&init);
        assert_eq!(repeat_num, 2772);
        let repeat = simulate_moons(&init, 2772);
        assert_eq!(init, repeat);
    }

    #[test]
    fn test_example2() {
        let input = vec![
            "<x=-8, y=-10, z=0>",
            "<x=5, y=5, z=10>",
            "<x=2, y=-7, z=3>",
            "<x=9, y=-8, z=-3>",
        ];
        let moons = parse_moons(input.iter().map(|s| s.to_string()).collect());
        let result_moons = simulate_moons(&moons, 100);
        let result = total_energy(&result_moons);
        assert_eq!(result, 1940);
        let repeat_num = find_repeat(&moons);
        assert_eq!(repeat_num, 4_686_774_924);
    }
}
