#[path = "common.rs"]
mod common;

fn simple_fuel_for_mass(m: i32) -> i32 {
    std::cmp::max(m / 3 - 2, 0)
}

fn fuel_for_mass(m: i32) -> i32 {
    let mut mass = m;
    let mut fuel = 0;
    while mass > 0 {
        let f = simple_fuel_for_mass(mass);
        fuel += f;
        mass = f;
    }
    fuel
}

#[allow(dead_code)]
fn main() {
    let input: Vec<i32> = common::get_lines()
        .into_iter()
        .map(|l| l.parse::<i32>().expect("could not parse number"))
        .collect();
    let result1: i32 = input.iter().map(|&m| simple_fuel_for_mass(m)).sum();
    let result2: i32 = input.iter().map(|&m| fuel_for_mass(m)).sum();
    println!("Part1: Total fuel required: {}", result1);
    println!("Part2: Total fuel required: {}", result2);
}
