use failure::Error;
use parse::parse_input;
use std::fmt::Display;

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
            Operand::Register(reg) => write!(f, "{}", reg),
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
    fn execute(&self, registers: &mut Registers) -> (Option<usize>, Option<u8>) {
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
                let value = (operand.evaluate(registers) % 8) as u8;
                (None, Some(value))
            }
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Instruction::*;
        match self {
            Divide(Operand::Register(Register::A), operand, Register::A) => {
                write!(f, "adv {}", operand)
            }
            Divide(Operand::Register(Register::A), operand, Register::B) => {
                write!(f, "bdv {}", operand)
            }
            Divide(Operand::Register(Register::A), operand, Register::C) => {
                write!(f, "cdv {}", operand)
            }
            Xor(Operand::Register(Register::B), Operand::Literal(n), Register::B) => {
                write!(f, "bxl {}", n)
            }
            Xor(Operand::Register(Register::B), Operand::Register(Register::C), Register::B) => {
                write!(f, "bxc  ")
            }
            Mod(operand, Register::B) => write!(f, "bst {}", operand),
            JumpNotZero(Operand::Literal(n)) => write!(f, "jnz {}", n),
            Out(operand) => write!(f, "out {}", operand),
            _ => write!(f, "{:?}", self),
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
        write!(
            f,
            "[{:3X},{:3X},{:3X}]",
            self.values[0], self.values[1], self.values[2]
        )
    }
}

struct Computer<'a> {
    registers: Registers,
    program: &'a [u8],
    instruction_pointer: usize,
    output: Vec<u8>,
    expected_output: Option<&'a [u8]>,
    debug: bool,
}

impl<'a> Computer<'a> {
    fn new(registers: Registers, program: &'a [u8], expected_output: Option<&'a [u8]>) -> Self {
        Computer {
            registers,
            program,
            instruction_pointer: 0,
            output: vec![],
            expected_output,
            debug: false,
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

    fn run(&mut self) -> bool {
        if self.debug {
            println!("Program: {:?}", self.program);
        }
        while let Some(instruction) = self.next_instruction() {
            if self.debug {
                println!(
                    "{:02} {} {}",
                    self.instruction_pointer, instruction, self.registers
                );
            }
            let (next, output) = instruction.execute(&mut self.registers);

            if let Some(instruction_pointer) = next {
                self.instruction_pointer = instruction_pointer;
            } else {
                self.instruction_pointer += 2;
            }

            if let Some(output) = output {
                if let Some(expected) = self.expected_output {
                    if expected.is_empty() || output != expected[0] {
                        return false;
                    }

                    self.expected_output = Some(&expected[1..]);
                }

                self.output.push(output);
            }
        }

        self.expected_output
            .map(|expected| expected.is_empty())
            .unwrap_or(true)
    }
}

fn get_output(registers: Registers, program: &[u8]) -> Vec<u8> {
    let mut computer = Computer::new(registers, program, None);
    computer.run();
    computer.output
}

fn program_has_output(registers: Registers, program: &[u8], output: &[u8]) -> bool {
    let mut computer = Computer::new(registers, program, Some(output));
    computer.run()
}

fn find_initial_reg_value(program: &[u8]) -> u64 {
    let mut candidates = vec![0];
    for idx in (0..program.len()).rev() {
        candidates = candidates
            .into_iter()
            .flat_map(|candidate| {
                (0..8).map(move |d| candidate * 8 + d).filter(|&candidate| {
                    get_output(Registers::new(candidate, 0, 0), program) == program[idx..]
                })
            })
            .collect()
    }

    let result = candidates
        .into_iter()
        .min()
        .expect("Failed to find solution");

    assert!(program_has_output(
        Registers::new(result, 0, 0),
        program,
        program
    ));

    result
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = (Registers, Box<[u8]>);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        parse_input(&data)
    }

    fn solve((registers, program): Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = get_output(registers, &program)
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let part2 = find_initial_reg_value(&program);
        (Some(part1), Some(part2.to_string()))
    }
}
