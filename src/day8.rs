use std::convert::TryFrom;
use std::convert::TryInto;
use std::str::FromStr;
use crate::input;

#[derive(PartialEq)]
enum Instruction {
    Acc(i16),
    Jmp(i16),
    NoOp(i16),
}

struct Computer {
    program: Vec<Instruction>,
    accumulator: i16,
    ip: i16,
}

impl Computer {
    fn parse_instruction(line: &String) -> Instruction {
        let ins = &line[0..3];
        if ins == "acc" {
            let value  = i16::from_str(&line[4..line.len()]).unwrap();
            return Instruction::Acc(value);
        } else if ins == "jmp" {
            let offset = i16::from_str(&line[4..line.len()]).unwrap();
            return Instruction::Jmp(offset);
        } else {
            let offset = i16::from_str(&line[4..line.len()]).unwrap();
            return Instruction::NoOp(offset);
        }
    }

    fn load_program(lines: &Vec<String>) -> Computer {
        let program = lines.iter().map(Computer::parse_instruction).collect();
        return Computer {
            program: program,
            accumulator: 0,
            ip: 0,
        }
    }

    fn next(&mut self) {
        let ip = usize::try_from(self.ip).unwrap();
        match self.program[ip] {
            Instruction::NoOp(_) => self.ip += 1,
            Instruction::Jmp(offset) => self.ip += offset,
            Instruction::Acc(amount) => {
                self.accumulator += amount;
                self.ip += 1
            }
        }
    }

    fn has_infinite_loop(&mut self) -> bool {
        let mut visited = Vec::new();
        visited.resize(self.program.len(), false);

        while self.ip < self.program.len().try_into().unwrap() &&
                !visited[usize::try_from(self.ip).unwrap()]
        {
            visited[usize::try_from(self.ip).unwrap()] = true;
            self.next();
        }
        if self.ip == self.program.len().try_into().unwrap() {
            return false;
        }
        return true;
    }
}


pub fn question1() -> String {
    let filename = "input/day8.txt";
    let input = input::lines_as(filename);
    let mut computer = Computer::load_program(&input);
    let mut visited = Vec::new();
    visited.resize(computer.program.len(), false);

    while !visited[usize::try_from(computer.ip).unwrap()] {
        visited[usize::try_from(computer.ip).unwrap()] = true;
        computer.next();
    }

    return format!("Day 8.1: accumulator value = {}", computer.accumulator);
}

pub fn question2() -> String {
    let filename = "input/day8.txt";
    let input = input::lines_as(filename);

    let mut last_flip_idx = 0;
    while last_flip_idx < input.len() {
        let mut computer = Computer::load_program(&input);

        match computer.program[last_flip_idx] {
            Instruction::Acc(_) => (),
            Instruction::Jmp(offset) => computer.program[last_flip_idx] = Instruction::NoOp(offset),
            Instruction::NoOp(offset) => computer.program[last_flip_idx] = Instruction::Jmp(offset)
        }

        if !computer.has_infinite_loop() {
            return format!("Day 8.2: accumulator value = {}", computer.accumulator);
        }
        last_flip_idx += 1;
    }
    return format!("Day 8.2: no program without infinite loop");
}