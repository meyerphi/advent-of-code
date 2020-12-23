mod common;

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.splitn(2, ' ');
        let op_str = split.next().ok_or("no operation")?;
        let arg_str = split.next().ok_or("no argument")?;
        let arg = arg_str
            .parse::<isize>()
            .map_err(|err| format!("invalid argument: {}", err))?;
        let instruction = match op_str {
            "nop" => Instruction::Nop(arg),
            "jmp" => Instruction::Jmp(arg),
            "acc" => Instruction::Acc(arg),
            _ => return Err(format!("invalid operation: {}", op_str)),
        };
        Ok(instruction)
    }
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

impl FromStr for Program {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<_> = s
            .lines()
            .map(|l| l.parse::<Instruction>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Program { instructions })
    }
}

enum ExecutionResult {
    Loop(isize),
    Terminate(isize),
    Error,
}

impl Program {
    fn execute(&self) -> ExecutionResult {
        let len = self.instructions.len();
        let mut visited = vec![false; len];
        let mut loc = 0;
        let mut acc = 0isize;
        loop {
            if loc > len {
                return ExecutionResult::Error;
            }
            if loc == len {
                return ExecutionResult::Terminate(acc);
            }
            if visited[loc] {
                return ExecutionResult::Loop(acc);
            }
            visited[loc] = true;
            match self.instructions[loc] {
                Instruction::Nop(_) => loc += 1,
                Instruction::Acc(arg) => {
                    acc += arg;
                    loc += 1
                }
                Instruction::Jmp(arg) => loc = (loc as isize + arg) as usize,
            };
        }
    }
}

fn part1(program: &Program) -> isize {
    match program.execute() {
        ExecutionResult::Loop(val) => val,
        _ => panic!("unexpected result"),
    }
}

fn swap_nop_jmp(program: &mut Program, loc: usize) {
    match program.instructions[loc] {
        Instruction::Nop(arg) => program.instructions[loc] = Instruction::Jmp(arg),
        Instruction::Jmp(arg) => program.instructions[loc] = Instruction::Nop(arg),
        _ => (),
    }
}

fn part2(mut program: &mut Program) -> isize {
    for loc in 0..program.instructions.len() {
        swap_nop_jmp(&mut program, loc);
        if let ExecutionResult::Terminate(val) = program.execute() {
            return val;
        }
        swap_nop_jmp(&mut program, loc);
    }
    panic!("could not fix program")
}

fn main() {
    let mut program = common::get_content().parse::<Program>().unwrap();
    println!("Part1: {}", part1(&program));
    println!("Part2: {}", part2(&mut program));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "nop +0\n\
        acc +1\n\
        jmp +4\n\
        acc +3\n\
        jmp -3\n\
        acc -99\n\
        acc +1\n\
        jmp -4\n\
        acc +6";

    #[test]
    fn test_part1() {
        let program = TEST_INPUT.parse::<Program>().unwrap();
        assert_eq!(part1(&program), 5);
    }

    #[test]
    fn test_part2() {
        let mut program = TEST_INPUT.parse::<Program>().unwrap();
        assert_eq!(part2(&mut program), 8);
    }
}
