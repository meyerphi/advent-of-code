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

struct ProgramIO {
    is: Sender<i32>,
    or: Receiver<Option<i32>>,
}

struct OutputIterator<'a> {
    io: &'a ProgramIO,
}

impl<'a> Iterator for OutputIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.io.next_output()
    }
}

impl ProgramIO {
    fn new(is: Sender<i32>, or: Receiver<Option<i32>>) -> ProgramIO {
        ProgramIO { is, or }
    }
    fn send_input(&self, i: i32) {
        self.is.send(i).expect("could not send input");
    }
    fn next_output(&self) -> Option<i32> {
        self.or.recv().expect("could not send input")
    }
    fn output_iter(&self) -> OutputIterator {
        OutputIterator { io: &self }
    }
    fn collect_outputs(&self) -> Vec<i32> {
        self.output_iter().collect()
    }
}

struct Program {
    program: Vec<i32>,
    ir: Receiver<i32>,
    os: Sender<Option<i32>>,
}

impl Program {
    fn new(program: &[i32], ir: Receiver<i32>, os: Sender<Option<i32>>) -> Program {
        Program {
            program: program.to_vec(),
            ir,
            os,
        }
    }

    fn run(&self) {
        let mut state = ProgramState {
            mem: self.program.clone(),
            pointer: 0,
        };
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
                    let i = self.ir.recv().unwrap();
                    state.write_value(0, i);
                    state.increase_pointer(2);
                }
                OpCode::Output => {
                    let o = state.fetch_value(0, &p);
                    self.os.send(Some(o)).unwrap();
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
    fn new(program: &[i32]) -> ProgramRunner {
        let (is, ir) = std::sync::mpsc::channel::<i32>();
        let (os, or) = std::sync::mpsc::channel::<Option<i32>>();
        ProgramRunner {
            program: Program::new(program, ir, os),
            io: ProgramIO::new(is, or),
        }
    }

    fn run_with(&self, inputs: &[i32]) -> Vec<i32> {
        for &i in inputs {
            self.io.send_input(i);
        }
        self.program.run();
        self.io.collect_outputs()
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
        let p = ProgramRunner::new(&program);

        let output1 = p.run_with(&[1]);
        println!("Part1: Program output is: {:?}", output1);

        let output2 = p.run_with(&[5]);
        println!("Part2: Program output is: {:?}", output2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threaded_run() {
        let ProgramRunner { program, io } = ProgramRunner::new(&[3, 0, 4, 0, 99]);
        let thread = std::thread::spawn(move || {
            program.run();
        });
        io.send_input(21);
        let out = io.next_output();
        let halt = io.next_output();
        assert_eq!(out, Some(21));
        assert_eq!(halt, None);
        thread.join().expect("could not join thread");
    }

    #[test]
    fn test_io() {
        let p = ProgramRunner::new(&[3, 0, 4, 0, 99]);
        let output = p.run_with(&[42]);
        assert_eq!(output, vec![42]);
    }

    #[test]
    fn test_param() {
        let p = ProgramRunner::new(&[1101, 100, -1, 4, 0]);
        let output = p.run_with(&[]);
        assert_eq!(output, vec![]);
    }

    #[test]
    fn test_compare_equal_position_mode() {
        let p = ProgramRunner::new(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        let output1 = p.run_with(&[7]);
        let output2 = p.run_with(&[8]);
        let output3 = p.run_with(&[9]);
        assert_eq!(output1, vec![0]);
        assert_eq!(output2, vec![1]);
        assert_eq!(output3, vec![0]);
    }

    #[test]
    fn test_compare_less_than_position_mode() {
        let p = ProgramRunner::new(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        let output1 = p.run_with(&[7]);
        let output2 = p.run_with(&[8]);
        let output3 = p.run_with(&[9]);
        assert_eq!(output1, vec![1]);
        assert_eq!(output2, vec![0]);
        assert_eq!(output3, vec![0]);
    }

    #[test]
    fn test_compare_equal_immediate_mode() {
        let p = ProgramRunner::new(&[3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        let output1 = p.run_with(&[7]);
        let output2 = p.run_with(&[8]);
        let output3 = p.run_with(&[9]);
        assert_eq!(output1, vec![0]);
        assert_eq!(output2, vec![1]);
        assert_eq!(output3, vec![0]);
    }

    #[test]
    fn test_compare_less_than_immediate_mode() {
        let p = ProgramRunner::new(&[3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        let output1 = p.run_with(&[7]);
        let output2 = p.run_with(&[8]);
        let output3 = p.run_with(&[9]);
        assert_eq!(output1, vec![1]);
        assert_eq!(output2, vec![0]);
        assert_eq!(output3, vec![0]);
    }

    #[test]
    fn test_jump_position() {
        let p = ProgramRunner::new(&[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]);
        let output1 = p.run_with(&[0]);
        let output2 = p.run_with(&[2]);
        assert_eq!(output1, vec![0]);
        assert_eq!(output2, vec![1]);
    }

    #[test]
    fn test_jump_immediate() {
        let p = ProgramRunner::new(&[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        let output1 = p.run_with(&[0]);
        let output2 = p.run_with(&[2]);
        assert_eq!(output1, vec![0]);
        assert_eq!(output2, vec![1]);
    }

    #[test]
    fn test_large() {
        let p = ProgramRunner::new(&[
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        let output1 = p.run_with(&[5]);
        let output2 = p.run_with(&[8]);
        let output3 = p.run_with(&[13]);
        assert_eq!(output1, vec![999]);
        assert_eq!(output2, vec![1000]);
        assert_eq!(output3, vec![1001]);
    }
}
