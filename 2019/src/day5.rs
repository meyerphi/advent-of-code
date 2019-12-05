mod common;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Op {
    Add,
    Mul,
}
#[derive(PartialEq, Eq, Copy, Clone)]
enum Cnd {
    True,
    False,
}
#[derive(PartialEq, Eq, Copy, Clone)]
enum Cmp {
    LessThan,
    Equal,
}
#[derive(PartialEq, Eq, Copy, Clone)]
enum OpCode {
    Arith(Op),
    Input,
    Output,
    JumpIf(Cnd),
    Compare(Cmp),
    Halt,
}
struct ParamOpCode {
    opcode: OpCode,
    params: [u8; 3],
}

fn parse_opcode(code: i32) -> ParamOpCode {
    let param1 = ((code / 100) % 10) as u8;
    let param2 = ((code / 1000) % 10) as u8;
    let param3 = ((code / 10000) % 10) as u8;
    let opcode = match code % 100 {
        1 => OpCode::Arith(Op::Add),
        2 => OpCode::Arith(Op::Mul),
        3 => OpCode::Input,
        4 => OpCode::Output,
        5 => OpCode::JumpIf(Cnd::True),
        6 => OpCode::JumpIf(Cnd::False),
        7 => OpCode::Compare(Cmp::LessThan),
        8 => OpCode::Compare(Cmp::Equal),
        99 => OpCode::Halt,
        _ => panic!("unknown opcode"),
    };
    ParamOpCode {
        opcode,
        params: [param1, param2, param3],
    }
}

struct ProgramState {
    mem: Vec<i32>,
    pointer: usize,
}

impl ProgramState {
    fn fetch_opcode(&self) -> i32 {
        self.mem[self.pointer]
    }

    fn increase_pointer(&mut self, position: usize) {
        self.pointer += position;
    }

    fn set_pointer(&mut self, position: usize) {
        self.pointer = position;
    }

    fn fetch_value(&self, position: usize, p: &ParamOpCode) -> i32 {
        if p.params[position] == 0 {
            self.mem[self.mem[self.pointer + position + 1] as usize]
        } else {
            self.mem[self.pointer + position + 1]
        }
    }

    fn write_value(&mut self, position: usize, value: i32) {
        let target = self.mem[self.pointer + position + 1] as usize;
        self.mem[target] = value;
    }
}

struct Program {
    program: Vec<i32>,
}

impl Program {
    fn new(program: &[i32]) -> Program {
        Program {
            program: program.to_vec(),
        }
    }

    fn run<'a>(&self, mut input: impl Iterator<Item = &'a i32>) -> Vec<i32> {
        let mut state = ProgramState {
            mem: self.program.clone(),
            pointer: 0,
        };
        let mut output: Vec<i32> = Vec::new();
        loop {
            let p = parse_opcode(state.fetch_opcode());
            match p.opcode {
                OpCode::Arith(op) => {
                    let x = state.fetch_value(0, &p);
                    let y = state.fetch_value(1, &p);
                    let z = match op {
                        Op::Add => x + y,
                        Op::Mul => x * y,
                    };
                    state.write_value(2, z);
                    state.increase_pointer(4);
                }
                OpCode::Input => {
                    let i = *input.next().unwrap();
                    state.write_value(0, i);
                    state.increase_pointer(2);
                }
                OpCode::Output => {
                    let o = state.fetch_value(0, &p);
                    output.push(o);
                    state.increase_pointer(2);
                }
                OpCode::JumpIf(condition) => {
                    let x = state.fetch_value(0, &p);
                    let matched = match condition {
                        Cnd::True => x != 0,
                        Cnd::False => x == 0,
                    };
                    if matched {
                        let y = state.fetch_value(1, &p);
                        state.set_pointer(y as usize);
                    } else {
                        state.increase_pointer(3);
                    }
                }
                OpCode::Compare(comparison) => {
                    let x = state.fetch_value(0, &p);
                    let y = state.fetch_value(1, &p);
                    let result = match comparison {
                        Cmp::LessThan => x < y,
                        Cmp::Equal => x == y,
                    };
                    if result {
                        state.write_value(2, 1);
                    } else {
                        state.write_value(2, 0);
                    }
                    state.increase_pointer(4);
                }
                OpCode::Halt => return output,
            }
        }
    }
}

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
        let program = Program::new(&program);

        let output1 = program.run([1].iter());
        println!("Part1: Program output is: {:?}", output1);

        let output2 = program.run([5].iter());
        println!("Part2: Program output is: {:?}", output2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io() {
        let program = Program::new(&[3, 0, 4, 0, 99]);
        let output = program.run([42].iter());
        assert_eq!(output, vec![42]);
    }

    #[test]
    fn test_param() {
        let program = Program::new(&[1101, 100, -1, 4, 0]);
        let output = program.run([].iter());
        assert_eq!(output, vec![]);
    }

    #[test]
    fn test_compare_equal_position_mode() {
        let program = Program::new(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        let output1 = program.run([7].iter());
        let output2 = program.run([8].iter());
        let output3 = program.run([9].iter());
        assert_eq!(output1, vec![0]);
        assert_eq!(output2, vec![1]);
        assert_eq!(output3, vec![0]);
    }

    #[test]
    fn test_compare_less_than_position_mode() {
        let program = Program::new(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        let output1 = program.run([7].iter());
        let output2 = program.run([8].iter());
        let output3 = program.run([9].iter());
        assert_eq!(output1, vec![1]);
        assert_eq!(output2, vec![0]);
        assert_eq!(output3, vec![0]);
    }

    #[test]
    fn test_compare_equal_immediate_mode() {
        let program = Program::new(&[3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        let output1 = program.run([7].iter());
        let output2 = program.run([8].iter());
        let output3 = program.run([9].iter());
        assert_eq!(output1, vec![0]);
        assert_eq!(output2, vec![1]);
        assert_eq!(output3, vec![0]);
    }

    #[test]
    fn test_compare_less_than_immediate_mode() {
        let program = Program::new(&[3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        let output1 = program.run([7].iter());
        let output2 = program.run([8].iter());
        let output3 = program.run([9].iter());
        assert_eq!(output1, vec![1]);
        assert_eq!(output2, vec![0]);
        assert_eq!(output3, vec![0]);
    }

    #[test]
    fn test_jump_position() {
        let program = Program::new(&[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]);
        let output1 = program.run([0].iter());
        let output2 = program.run([2].iter());
        assert_eq!(output1, vec![0]);
        assert_eq!(output2, vec![1]);
    }

    #[test]
    fn test_jump_immediate() {
        let program = Program::new(&[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        let output1 = program.run([0].iter());
        let output2 = program.run([2].iter());
        assert_eq!(output1, vec![0]);
        assert_eq!(output2, vec![1]);
    }

    #[test]
    fn test_large() {
        let program = Program::new(&[
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        let output1 = program.run([5].iter());
        let output2 = program.run([8].iter());
        let output3 = program.run([13].iter());
        assert_eq!(output1, vec![999]);
        assert_eq!(output2, vec![1000]);
        assert_eq!(output3, vec![1001]);
    }
}
