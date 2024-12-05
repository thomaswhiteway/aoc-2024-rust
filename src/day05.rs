use std::collections::{HashMap, HashSet};

use failure::{err_msg, Error};

use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::{many0, separated_list0},
    sequence::{separated_pair, terminated},
};

use crate::parsers::unsigned;

fn parse_input(input: &str) -> Result<(Rules, Box<[Box<[u32]>]>), Error> {
    let rule = separated_pair(unsigned, tag("|"), unsigned);
    let rules = map(many0(terminated(rule, newline)), Rules::new);

    let page_numbers = map(separated_list0(tag(","), unsigned), |v| {
        v.into_boxed_slice()
    });
    let all_page_numbers = map(many0(terminated(page_numbers, newline)), |v| {
        v.into_boxed_slice()
    });

    separated_pair(rules, newline, all_page_numbers)(input)
        .map(|(_, input)| input)
        .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
}

#[derive(Clone)]
pub struct Rules {
    blocked_by: HashMap<u32, HashSet<u32>>,
}

impl Rules {
    fn new(rules: Vec<(u32, u32)>) -> Self {
        let mut blocked_by: HashMap<u32, HashSet<u32>> = HashMap::new();

        for (blocker, blockee) in rules {
            blocked_by.entry(blockee).or_default().insert(blocker);
        }

        Rules { blocked_by }
    }

    fn in_sequence(&self, page_numbers: &[u32]) -> bool {
        let mut disallowed = HashSet::new();

        for page in page_numbers {
            if disallowed.contains(page) {
                return false;
            }

            if let Some(blockers) = self.blocked_by.get(page) {
                for blocker in blockers {
                    disallowed.insert(blocker);
                }
            }
        }

        true
    }
}

fn find_mid_number(page_numbers: &[u32]) -> u32 {
    page_numbers[page_numbers.len() / 2]
}

fn find_mid_numbers(rules: Rules, sequences: &[Box<[u32]>]) -> u32 {
    sequences
        .iter()
        .filter(|page_numbers| rules.in_sequence(page_numbers))
        .map(|page_numbers| find_mid_number(page_numbers))
        .sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = (Rules, Box<[Box<[u32]>]>);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        parse_input(&data)
    }

    fn solve((rules, page_numbers): Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_mid_numbers(rules, &page_numbers);

        (Some(part1.to_string()), None)
    }
}
