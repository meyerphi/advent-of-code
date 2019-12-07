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

    fn with_io(program: &[i32]) -> (Program, ProgramIO) {
        let (is, ir) = std::sync::mpsc::channel::<i32>();
        let (os, or) = std::sync::mpsc::channel::<Option<i32>>();
        (Program::new(program, ir, os), ProgramIO::new(is, or))
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

fn run_amplifiers(program: &[i32], phase_settings: &[i32; 5]) -> i32 {
    let (a, a_io) = Program::with_io(&program);
    let (b, b_io) = Program::with_io(&program);
    let (c, c_io) = Program::with_io(&program);
    let (d, d_io) = Program::with_io(&program);
    let (e, e_io) = Program::with_io(&program);

    let a_thread = std::thread::spawn(move || {
        a.run();
    });
    let b_thread = std::thread::spawn(move || {
        b.run();
    });
    let c_thread = std::thread::spawn(move || {
        c.run();
    });
    let d_thread = std::thread::spawn(move || {
        d.run();
    });
    let e_thread = std::thread::spawn(move || {
        e.run();
    });

    a_io.send_input(phase_settings[0]);
    b_io.send_input(phase_settings[1]);
    c_io.send_input(phase_settings[2]);
    d_io.send_input(phase_settings[3]);
    e_io.send_input(phase_settings[4]);

    a_io.send_input(0);
    b_io.send_input(a_io.next_output().unwrap());
    c_io.send_input(b_io.next_output().unwrap());
    d_io.send_input(c_io.next_output().unwrap());
    e_io.send_input(d_io.next_output().unwrap());
    let result = e_io
        .next_output()
        .expect("last amplifier produced no output");

    a_thread.join().expect("could not join thread");
    b_thread.join().expect("could not join thread");
    c_thread.join().expect("could not join thread");
    d_thread.join().expect("could not join thread");
    e_thread.join().expect("could not join thread");
    result
}

fn next_phase(phase: &mut [i32; 5]) -> bool {
    // Find non-increasing suffix
    let mut i: usize = phase.len() - 1;
    while i > 0 && phase[i - 1] >= phase[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }

    // Find successor to pivot
    let mut j: usize = phase.len() - 1;
    while phase[j] <= phase[i - 1] {
        j -= 1;
    }
    phase.swap(i - 1, j);

    // Reverse suffix
    phase[i..].reverse();
    true
}

fn find_best_phase_setting(program: &[i32]) -> i32 {
    let mut phase: [i32; 5] = [0, 1, 2, 3, 4];
    let mut max = -1;
    loop {
        let output = run_amplifiers(&program, &phase);
        max = std::cmp::max(max, output);
        if !next_phase(&mut phase) {
            break;
        }
    }
    max
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
        let result1 = find_best_phase_setting(&program);
        println!("Part1: Highest signal is {}", result1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amplifiers_example1() {
        let program = [
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let phases = [4, 3, 2, 1, 0];
        let result = run_amplifiers(&program, &phases);
        assert_eq!(result, 43210);
        let optimal = find_best_phase_setting(&program);
        assert_eq!(optimal, 43210);
    }

    #[test]
    fn test_amplifiers_example2() {
        let program = [
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let phases = [0, 1, 2, 3, 4];
        let result = run_amplifiers(&program, &phases);
        assert_eq!(result, 54321);
        let optimal = find_best_phase_setting(&program);
        assert_eq!(optimal, 54321);
    }

    #[test]
    fn test_amplifiers_example3() {
        let program = [
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let phases = [1, 0, 4, 3, 2];
        let result = run_amplifiers(&program, &phases);
        assert_eq!(result, 65210);
        let optimal = find_best_phase_setting(&program);
        assert_eq!(optimal, 65210);
    }
}
