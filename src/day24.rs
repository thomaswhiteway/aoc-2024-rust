use std::collections::{HashMap, HashSet};

use failure::{err_msg, Error};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline},
    combinator::{all_consuming, map, value},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

use crate::parsers::unsigned;

#[derive(Debug, Clone, Copy)]
enum Operation {
    And,
    Xor,
    Or,
}

impl Operation {
    fn resolve(self, v1: u8, v2: u8) -> u8 {
        use Operation::*;
        match self {
            And => v1 & v2,
            Or => v1 | v2,
            Xor => v1 ^ v2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Gate {
    inputs: [String; 2],
    operation: Operation,
    output: String,
}

impl Gate {
    fn resolve(&self, wires: &HashMap<&str, u8>) -> Option<u8> {
        if let (Some(v1), Some(v2)) = (
            wires.get(self.inputs[0].as_str()),
            wires.get(self.inputs[1].as_str()),
        ) {
            Some(self.operation.resolve(*v1, *v2))
        } else {
            None
        }
    }
}

fn wire(input: &str) -> IResult<&str, String> {
    map(alphanumeric1, str::to_string)(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    alt((
        value(Operation::And, tag("AND")),
        value(Operation::Xor, tag("XOR")),
        value(Operation::Or, tag("OR")),
    ))(input)
}

fn gate(input: &str) -> IResult<&str, Gate> {
    let (input, gate1) = wire(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, operation) = operation(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, gate2) = wire(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, output) = wire(input)?;

    Ok((
        input,
        Gate {
            inputs: [gate1, gate2],
            operation,
            output,
        },
    ))
}

fn gates(input: &str) -> IResult<&str, Box<[Gate]>> {
    map(many1(terminated(gate, newline)), Vec::into_boxed_slice)(input)
}

fn get_output(inputs: &HashMap<String, u8>, gates: &[Gate]) -> u64 {
    let mut wires = inputs
        .iter()
        .map(|(name, val)| (name.as_str(), *val))
        .collect();
    let mut connections: HashMap<&str, Vec<&Gate>> = HashMap::new();

    for gate in gates {
        connections.entry(&gate.inputs[0]).or_default().push(gate);
        connections.entry(&gate.inputs[1]).or_default().push(gate);
    }

    let mut updated: HashSet<&str> = inputs.keys().map(String::as_str).collect();

    while !updated.is_empty() {
        let mut new_updated = HashSet::new();

        for wire in updated {
            if let Some(gates) = connections.get(wire) {
                for gate in gates {
                    if let Some(output) = gate.resolve(&wires) {
                        wires.insert(gate.output.as_str(), output);
                        new_updated.insert(gate.output.as_str());
                    }
                }
            }
        }

        updated = new_updated;
    }

    let mut z_wires: Vec<_> = wires
        .iter()
        .filter(|(name, _)| name.starts_with("z"))
        .collect();
    z_wires.sort();
    z_wires
        .iter()
        .enumerate()
        .map(|(index, (_, &val))| 2u64.pow(index as u32) * val as u64)
        .sum()
}
pub struct Solver {}
impl super::Solver for Solver {
    type Problem = (HashMap<String, u8>, Box<[Gate]>);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let input = separated_pair(wire, tag(": "), unsigned);
        let inputs = map(many1(terminated(input, newline)), |vals| {
            vals.into_iter().collect()
        });

        all_consuming(separated_pair(inputs, newline, gates))(&data)
            .map(|(_, result)| result)
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
    }

    fn solve((inputs, connections): Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = get_output(&inputs, &connections);
        (Some(part1.to_string()), None)
    }
}
