use std::fmt::Display;
use failure::Error;
use parse::parse_input;

mod parse {
    use crate::parsers::unsigned;
    use failure::{err_msg, Error};
    use nom::combinator::{all_consuming, map};
    use nom::sequence::{delimited, separated_pair};
    use nom::{bytes::complete::tag, character::complete::newline, multi::separated_list1};
    use nom::{sequence::tuple, IResult};

    use super::Registers;

    fn registers(input: &str) -> IResult<&str, Registers> {
        map(
            tuple((
                delimited(tag("Register A: "), unsigned, newline),
                delimited(tag("Register B: "), unsigned, newline),
                delimited(tag("Register C: "), unsigned, newline),
            )),
            |(a, b, c)| Registers::new(a, b, c),
        )(input)
    }

    fn instructions(input: &str) -> IResult<&str, Box<[u8]>> {
        delimited(
            tag("Program: "),
            map(separated_list1(tag(","), unsigned), |v| {
                v.into_boxed_slice()
            }),
            newline,
        )(input)
    }

    pub(super) fn parse_input(input: &str) -> Result<(Registers, Box<[u8]>), Error> {
        all_consuming(separated_pair(registers, newline, instructions))(input)
            .map(|(_, val)| val)
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
    }
}

#[derive(Clone, Copy, Debug)]
enum Register {
    A,
    B,
    C,
}

impl Register {
    fn index(self) -> usize {
        match self {
            Register::A => 0,
            Register::B => 1,
            Register::C => 2,
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::A => write!(f, "A"),
            Register::B => write!(f, "B"),
            Register::C => write!(f, "C"),
        }
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Literal(u8),
    Register(Register),
}

impl Operand {
    fn combo(val: u8) -> Self {
        match val {
            0..=3 => Operand::Literal(val),
            4 => Operand::Register(Register::A),
            5 => Operand::Register(Register::B),
            6 => Operand::Register(Register::C),
            _ => panic!("Invalid combo operand {}", val),
        }
    }

    fn evaluate(&self, registers: &Registers) -> u64 {
        match self {
            Operand::Literal(val) => *val as u64,
            Operand::Register(reg) => registers.get(*reg),
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Literal(n) => write!(f, "{}", n),
            Operand::Register(reg) => write!(f, "{}", reg)
        }
    }
}

impl From<Register> for Operand {
    fn from(value: Register) -> Self {
        Operand::Register(value)
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Divide(Operand, Operand, Register),
    Xor(Operand, Operand, Register),
    Mod(Operand, Register),
    JumpNotZero(Operand),
    Out(Operand),
}

impl Instruction {
    fn execute(&self, registers: &mut Registers) -> (Option<usize>, Option<u64>) {
        use Instruction::*;
        match self {
            Divide(numerator, denominator, store) => {
                let result = numerator.evaluate(registers)
                    / 2u64.pow(denominator.evaluate(registers) as u32);
                registers.set(*store, result);
                (None, None)
            }
            Xor(left, right, store) => {
                let result = left.evaluate(registers) ^ right.evaluate(registers);
                registers.set(*store, result);
                (None, None)
            }
            Mod(operand, store) => {
                let result = operand.evaluate(registers) % 8;
                registers.set(*store, result);
                (None, None)
            }
            JumpNotZero(operand) => {
                if registers.get(Register::A) != 0 {
                    (Some(operand.evaluate(registers) as usize), None)
                } else {
                    (None, None)
                }
            }
            Out(operand) => {
                let value = operand.evaluate(registers) % 8;
                (None, Some(value))
            }
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Instruction::*;
        match self {
            Divide(Operand::Register(Register::A), operand, Register::A) => write!(f, "adv {}", operand),
            Divide(Operand::Register(Register::A), operand, Register::B) => write!(f, "bdv {}", operand),
            Divide(Operand::Register(Register::A), operand, Register::C) => write!(f, "cdv {}", operand),
            Xor(Operand::Register(Register::B), Operand::Literal(n), Register::B) => write!(f, "bxl {}", n),
            Xor(Operand::Register(Register::B), Operand::Register(Register::C), Register::B) => write!(f, "bxc  "),
            Mod(operand, Register::B) => write!(f, "bst {}", operand),
            JumpNotZero(Operand::Literal(n)) => write!(f, "jnz {}", n),
            Out(operand) => write!(f, "out {}", operand),
            _ => write!(f, "{:?}", self)
        }
    }
}

pub struct Registers {
    values: [u64; 3],
}

impl Registers {
    fn new(a: u64, b: u64, c: u64) -> Self {
        Registers { values: [a, b, c] }
    }

    fn get(&self, register: Register) -> u64 {
        self.values[register.index()]
    }

    fn set(&mut self, register: Register, value: u64) {
        self.values[register.index()] = value;
    }
}

impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:3},{:3},{:3}]", self.values[0], self.values[1], self.values[2])
    }
}

struct Computer<'a> {
    registers: Registers,
    program: &'a [u8],
    instruction_pointer: usize,
    output: Vec<u64>,
}

impl<'a> Computer<'a> {
    fn new(registers: Registers, program: &'a [u8]) -> Self {
        Computer {
            registers,
            program,
            instruction_pointer: 0,
            output: vec![],
        }
    }

    fn parse_instruction(&self, operator: u8, operand: u8) -> Instruction {
        use Instruction::*;
        match operator {
            0 => Divide(Register::A.into(), Operand::combo(operand), Register::A),
            1 => Xor(Register::B.into(), Operand::Literal(operand), Register::B),
            2 => Mod(Operand::combo(operand), Register::B),
            3 => JumpNotZero(Operand::Literal(operand)),
            4 => Xor(Register::B.into(), Register::C.into(), Register::B),
            5 => Out(Operand::combo(operand)),
            6 => Divide(Register::A.into(), Operand::combo(operand), Register::B),
            7 => Divide(Register::A.into(), Operand::combo(operand), Register::C),
            _ => panic!("Invalid operator {}", operator),
        }
    }

    fn next_instruction(&self) -> Option<Instruction> {
        if self.instruction_pointer < self.program.len() {
            let operator = self.program[self.instruction_pointer];
            let operand = self.program[self.instruction_pointer + 1];

            Some(self.parse_instruction(operator, operand))
        } else {
            None
        }
    }

    fn run(&mut self) {
        println!("{:?}", self.program);
        while let Some(instruction) = self.next_instruction() {
            println!("{:02}: {:16} {}", self.instruction_pointer, instruction, self.registers);
            let (next, output) = instruction.execute(&mut self.registers);

            if let Some(instruction_pointer) = next {
                self.instruction_pointer = instruction_pointer;
            } else {
                self.instruction_pointer += 2;
            }

            if let Some(output) = output {
                self.output.push(output);
            }
        }
    }
}

fn get_output(registers: Registers, instructions: &[u8]) -> String {
    let mut computer = Computer::new(registers, instructions);
    computer.run();
    computer
        .output
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = (Registers, Box<[u8]>);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        parse_input(&data)
    }

    fn solve((registers, instructions): Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = get_output(registers, &instructions);

        (Some(part1), None)
    }
}
