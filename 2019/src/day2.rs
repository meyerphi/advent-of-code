#[path = "common.rs"]
mod common;

enum Op {
    Add,
    Mul,
}
enum OpCode {
    Arith(Op),
    Halt,
}

fn parse_opcode(opcode: i32) -> OpCode {
    match opcode {
        1 => OpCode::Arith(Op::Add),
        2 => OpCode::Arith(Op::Mul),
        99 => OpCode::Halt,
        _ => panic!("unknown opcode"),
    }
}

fn part1(program: &[i32]) -> i32 {
    let mut state = program.to_vec();
    let mut pointer = 0;
    loop {
        let opcode = parse_opcode(state[pointer]);
        match opcode {
            OpCode::Arith(op) => {
                let x = state[state[pointer + 1] as usize];
                let y = state[state[pointer + 2] as usize];
                let target = state[pointer + 3] as usize;
                let z = match op {
                    Op::Add => x + y,
                    Op::Mul => x * y,
                };
                state[target] = z;
                pointer += 4;
            }
            OpCode::Halt => return state[0],
        }
    }
}

fn replace(program: &[i32], one: i32, two: i32) -> Vec<i32> {
    let mut state = program.to_vec();
    state[1] = one;
    state[2] = two;
    state
}

#[allow(dead_code)]
fn main() {
    let input: Vec<Vec<i32>> = common::get_lines()
        .into_iter()
        .map(|l| {
            l.split(',')
                .map(|i| i.parse::<i32>().expect("could not parse number"))
                .collect()
        })
        .collect();
    for program in input {
        let replaced = replace(&program, 12, 2);
        let result1 = part1(&replaced);
        println!("Part1: Program output is: {}", result1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]), 3500);
    }

    #[test]
    fn test_part1_small() {
        assert_eq!(part1(&[1, 0, 0, 0, 99]), 2);
        assert_eq!(part1(&[2, 5, 0, 0, 99, 3]), 6);
        assert_eq!(part1(&[2, 4, 4, 0, 99, 0]), 9801);
        assert_eq!(part1(&[1, 1, 1, 4, 99, 5, 6, 0, 99]), 30);
    }
}
