mod common;
use std::sync::mpsc::channel;
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

struct Program {
    program: Vec<i32>,
    ir: Receiver<Option<i32>>,
    os: Sender<Option<i32>>,
}

impl Program {
    fn new(program: &[i32], ir: Receiver<Option<i32>>, os: Sender<Option<i32>>) -> Program {
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
                    // panic if there is no more input
                    let i = self.ir.recv().unwrap().unwrap();
                    state.write_value(0, i);
                    state.increase_pointer(2);
                }
                OpCode::Output => {
                    let o = state.fetch_value(0, &p);
                    // ignore send errors
                    let _ = self.os.send(Some(o));
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
                    // ignore send errors
                    let _ = self.os.send(None);
                    break;
                }
            }
        }
    }
}

fn run_amplifiers(program: &[i32], phase_settings: &[i32; 5], feedback: bool) -> i32 {
    let (input, a_in) = channel::<Option<i32>>();
    let (a_out, b_in) = channel::<Option<i32>>();
    let (b_out, c_in) = channel::<Option<i32>>();
    let (c_out, d_in) = channel::<Option<i32>>();
    let (d_out, e_in) = channel::<Option<i32>>();
    let (e_out, output) = channel::<Option<i32>>();

    input.send(Some(phase_settings[0])).unwrap();
    a_out.send(Some(phase_settings[1])).unwrap();
    b_out.send(Some(phase_settings[2])).unwrap();
    c_out.send(Some(phase_settings[3])).unwrap();
    d_out.send(Some(phase_settings[4])).unwrap();

    let a = Program::new(&program, a_in, a_out);
    let b = Program::new(&program, b_in, b_out);
    let c = Program::new(&program, c_in, c_out);
    let d = Program::new(&program, d_in, d_out);
    let e = Program::new(&program, e_in, e_out);

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

    input.send(Some(0)).unwrap();
    let result = if feedback {
        let mut last_output = None;
        while let Some(out) = output.recv().unwrap() {
            last_output = Some(out);
            // ignore error in case first thread has already terminated
            let _ = input.send(Some(out));
        }
        last_output
    } else {
        output.recv().unwrap()
    };

    a_thread.join().expect("could not join thread");
    b_thread.join().expect("could not join thread");
    c_thread.join().expect("could not join thread");
    d_thread.join().expect("could not join thread");
    e_thread.join().expect("could not join thread");

    result.unwrap()
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

fn find_best_phase_setting(program: &[i32], feedback: bool) -> i32 {
    let mut phase: [i32; 5] = if feedback {
        [5, 6, 7, 8, 9]
    } else {
        [0, 1, 2, 3, 4]
    };
    let mut max = -1;
    loop {
        let output = run_amplifiers(&program, &phase, feedback);
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
        let result1 = find_best_phase_setting(&program, false);
        println!("Part1: Highest signal is {}", result1);

        let result2 = find_best_phase_setting(&program, true);
        println!("Part2: Highest signal is {}", result2);
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
        let result = run_amplifiers(&program, &phases, false);
        assert_eq!(result, 43210);
        let optimal = find_best_phase_setting(&program, false);
        assert_eq!(optimal, 43210);
    }

    #[test]
    fn test_amplifiers_example2() {
        let program = [
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let phases = [0, 1, 2, 3, 4];
        let result = run_amplifiers(&program, &phases, false);
        assert_eq!(result, 54321);
        let optimal = find_best_phase_setting(&program, false);
        assert_eq!(optimal, 54321);
    }

    #[test]
    fn test_amplifiers_example3() {
        let program = [
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let phases = [1, 0, 4, 3, 2];
        let result = run_amplifiers(&program, &phases, false);
        assert_eq!(result, 65210);
        let optimal = find_best_phase_setting(&program, false);
        assert_eq!(optimal, 65210);
    }

    #[test]
    fn test_feedback_example1() {
        let program = [
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let phases = [9, 8, 7, 6, 5];
        let result = run_amplifiers(&program, &phases, true);
        assert_eq!(result, 139_629_729);
        let optimal = find_best_phase_setting(&program, true);
        assert_eq!(optimal, 139_629_729);
    }

    #[test]
    fn test_feedback_example2() {
        let program = [
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let phases = [9, 7, 8, 5, 6];
        let result = run_amplifiers(&program, &phases, true);
        assert_eq!(result, 18216);
        let optimal = find_best_phase_setting(&program, true);
        assert_eq!(optimal, 18216);
    }
}
