use crate::parsers::unsigned;

use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
};

use failure::{err_msg, Error};

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    fn basic() -> Box<[Operator]> {
        Box::new([Operator::Add, Operator::Mul])
    }

    fn extended() -> Box<[Operator]> {
        Box::new([Operator::Add, Operator::Mul, Operator::Concat])
    }

    fn apply(self, left: u64, right: u64) -> u64 {
        match self {
            Operator::Add => left + right,
            Operator::Mul => left * right,
            Operator::Concat => left * 10u64.pow(right.ilog10() + 1) + right,
        }
    }
}

pub struct Equation {
    result: u64,
    values: Box<[u64]>,
}

impl Equation {
    fn has_solution(&self, operators: &[Operator]) -> bool {
        let mut stack = vec![(self.values[0], &self.values[1..])];

        while let Some((total, remaining)) = stack.pop() {
            if !remaining.is_empty() {
                for op in operators {
                    stack.push((op.apply(total, remaining[0]), &remaining[1..]));
                }
            } else if total == self.result {
                return true;
            }
        }

        false
    }
}

fn find_total_valid_sum(equations: &[Equation], operators: &[Operator]) -> u64 {
    equations
        .iter()
        .filter(|eq| eq.has_solution(operators))
        .map(|eq| eq.result)
        .sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Box<[Equation]>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let values = map(separated_list1(tag(" "), unsigned), |v| {
            v.into_boxed_slice()
        });
        let equation = map(
            separated_pair(unsigned, tag(": "), values),
            |(result, values)| Equation { result, values },
        );
        let mut equations = map(many1(terminated(equation, newline)), |v| {
            v.into_boxed_slice()
        });

        equations(&data)
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
            .map(|(_, equations)| equations)
    }

    fn solve(equations: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_total_valid_sum(&equations, &Operator::basic());
        let part2 = find_total_valid_sum(&equations, &Operator::extended());
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
