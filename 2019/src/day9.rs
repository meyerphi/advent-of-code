mod common;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

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
    AdjustRelativeBase,
    Halt,
}
#[derive(PartialEq, Eq, Copy, Clone)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

struct ProgramState {
    mem: Vec<i64>,
    pointer: usize,
    relative_base: usize,
}

impl ProgramState {
    fn from_program(program: &Program) -> ProgramState {
        ProgramState {
            mem: program.program.clone(),
            pointer: 0,
            relative_base: 0,
        }
    }

    fn fetch_opcode(&self) -> OpCode {
        let opcode = self.mem[self.pointer] % 100;
        match opcode {
            1 => OpCode::Arith(Op::Add),
            2 => OpCode::Arith(Op::Mul),
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIf(Cnd::True),
            6 => OpCode::JumpIf(Cnd::False),
            7 => OpCode::Compare(Cmp::LessThan),
            8 => OpCode::Compare(Cmp::Equal),
            9 => OpCode::AdjustRelativeBase,
            99 => OpCode::Halt,
            _ => panic!("unknown opcode"),
        }
    }

    fn fetch_mode(&self, arg: usize) -> Mode {
        let mode = (self.mem[self.pointer] / 10i64.pow(arg as u32 + 2)) % 10;
        match mode {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("unknown mode"),
        }
    }

    fn increase_pointer(&mut self, adjustment: usize) {
        self.pointer += adjustment;
    }

    fn set_pointer(&mut self, position: usize) {
        self.pointer = position;
    }

    fn increase_relative_base(&mut self, adjustment: i64) {
        self.relative_base = (self.relative_base as i64 + adjustment) as usize;
    }

    fn ensure_memory_available(&mut self, position: usize) {
        if position >= self.mem.len() {
            self.mem.resize(position + 1, 0);
        }
    }

    fn fetch_position(&mut self, arg: usize) -> usize {
        let mode = self.fetch_mode(arg);
        let base = self.pointer + arg + 1;
        let position = match mode {
            Mode::Position => self.mem[base] as usize,
            Mode::Immediate => base,
            Mode::Relative => (self.relative_base as i64 + self.mem[base]) as usize,
        };
        self.ensure_memory_available(position);
        position
    }

    fn fetch_value(&mut self, arg: usize) -> i64 {
        let position = self.fetch_position(arg);
        self.mem[position]
    }

    fn write_value(&mut self, arg: usize, value: i64) {
        let position = self.fetch_position(arg);
        self.mem[position] = value;
    }
}

struct ProgramIO {
    is: Sender<i64>,
    or: Receiver<Option<i64>>,
}

struct OutputIterator<'a> {
    io: &'a ProgramIO,
}

impl<'a> Iterator for OutputIterator<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.io.next_output()
    }
}

impl ProgramIO {
    fn new(is: Sender<i64>, or: Receiver<Option<i64>>) -> ProgramIO {
        ProgramIO { is, or }
    }
    fn send_input(&self, i: i64) {
        self.is.send(i).expect("could not send input");
    }
    fn next_output(&self) -> Option<i64> {
        self.or.recv().expect("could not receive output")
    }
    fn output_iter(&self) -> OutputIterator {
        OutputIterator { io: &self }
    }
    fn collect_outputs(&self) -> Vec<i64> {
        self.output_iter().collect()
    }
}

struct Program {
    program: Vec<i64>,
    ir: Receiver<i64>,
    os: Sender<Option<i64>>,
}

impl Program {
    fn new(program: &[i64], ir: Receiver<i64>, os: Sender<Option<i64>>) -> Program {
        Program {
            program: program.to_vec(),
            ir,
            os,
        }
    }

    fn run(&self) {
        let mut state = ProgramState::from_program(&self);
        loop {
            match state.fetch_opcode() {
                OpCode::Arith(op) => {
                    let x = state.fetch_value(0);
                    let y = state.fetch_value(1);
                    let z = match op {
                        Op::Add => x + y,
                        Op::Mul => x * y,
                    };
                    state.write_value(2, z);
                    state.increase_pointer(4);
                }
                OpCode::Input => {
                    let i = self.ir.recv().unwrap();
                    state.write_value(0, i);
                    state.increase_pointer(2);
                }
                OpCode::Output => {
                    let o = state.fetch_value(0);
                    self.os.send(Some(o)).unwrap();
                    state.increase_pointer(2);
                }
                OpCode::JumpIf(condition) => {
                    let x = state.fetch_value(0);
                    let matched = match condition {
                        Cnd::True => x != 0,
                        Cnd::False => x == 0,
                    };
                    if matched {
                        let y = state.fetch_value(1);
                        state.set_pointer(y as usize);
                    } else {
                        state.increase_pointer(3);
                    }
                }
                OpCode::Compare(comparison) => {
                    let x = state.fetch_value(0);
                    let y = state.fetch_value(1);
                    let result = match comparison {
                        Cmp::LessThan => x < y,
                        Cmp::Equal => x == y,
                    };
                    state.write_value(2, result as i64);
                    state.increase_pointer(4);
                }
                OpCode::AdjustRelativeBase => {
                    let x = state.fetch_value(0);
                    state.increase_relative_base(x);
                    state.increase_pointer(2);
                }
                OpCode::Halt => {
                    self.os.send(None).expect("could not send halt output");
                    break;
                }
            }
        }
    }
}

struct ProgramRunner {
    program: Program,
    io: ProgramIO,
}

impl ProgramRunner {
    fn new(program: &[i64]) -> ProgramRunner {
        let (is, ir) = std::sync::mpsc::channel::<i64>();
        let (os, or) = std::sync::mpsc::channel::<Option<i64>>();
        ProgramRunner {
            program: Program::new(program, ir, os),
            io: ProgramIO::new(is, or),
        }
    }

    fn run_with(&self, inputs: &[i64]) -> Vec<i64> {
        for &i in inputs {
            self.io.send_input(i);
        }
        self.program.run();
        self.io.collect_outputs()
    }
}

fn main() {
    let input: Vec<Vec<i64>> = common::get_lines()
        .into_iter()
        .map(|l| {
            l.split(',')
                .map(|i| i.parse::<i64>().expect("could not parse number"))
                .collect()
        })
        .collect();
    for program in input {
        let p = ProgramRunner::new(&program);

        let output1 = p.run_with(&[1]);
        println!("Part1: Program output is: {:?}", output1);

        let output2 = p.run_with(&[2]);
        println!("Part2: Program output is: {:?}", output2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quine() {
        let intcode = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let p = ProgramRunner::new(&intcode);
        let output = p.run_with(&[]);
        assert_eq!(output, intcode);
    }

    #[test]
    fn test_16_bit_number() {
        let intcode = vec![1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0];
        let p = ProgramRunner::new(&intcode);
        let output = p.run_with(&[]);
        assert_eq!(output, [1_219_070_632_396_864]);
    }

    #[test]
    fn test_large_number() {
        let intcode = vec![104, 1_125_899_906_842_624, 99];
        let p = ProgramRunner::new(&intcode);
        let output = p.run_with(&[]);
        assert_eq!(output, [1_125_899_906_842_624]);
    }
}
