mod common;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Op {
    Add,
    Mul,
}
#[derive(PartialEq, Eq, Copy, Clone)]
enum OpCode {
    Arith(Op),
    Input,
    Output,
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
    state: ProgramState,
}

impl Program {
    fn new(program: &[i32]) -> Program {
        Program { state: ProgramState { mem: program.to_vec(), pointer: 0 } }
    }

    fn run<'a>(&mut self, mut input: impl Iterator<Item = &'a i32>) -> Vec<i32> {
        let mut output: Vec<i32> = Vec::new();
        loop {
            let p = parse_opcode(self.state.fetch_opcode());
            match p.opcode {
                OpCode::Arith(op) => {
                    let x = self.state.fetch_value(0, &p);
                    let y = self.state.fetch_value(1, &p);
                    let z = match op {
                        Op::Add => x + y,
                        Op::Mul => x * y,
                    };
                    self.state.write_value(2, z);
                    self.state.increase_pointer(4);
                }
                OpCode::Input => {
                    let i = *input.next().unwrap();
                    self.state.write_value(0, i);
                    self.state.increase_pointer(2);
                }
                OpCode::Output => {
                    let o = self.state.fetch_value(0, &p);
                    output.push(o);
                    self.state.increase_pointer(2);
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
        let mut program = Program::new(&program);
        let output = program.run([1].iter());
        println!("Part1: Program output is: {:?}", output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io() {
        let mut program = Program::new(&[3, 0, 4, 0, 99]);
        let output = program.run([42].iter());
        assert_eq!(output, vec![42]);
    }

    #[test]
    fn test_param() {
        let mut program = Program::new(&[1101, 100, -1, 4, 0]);
        let output = program.run([].iter());
        assert_eq!(output, vec![]);
        assert_eq!(program.state.mem, vec![1101, 100, -1, 4, 99]);
    }
}
