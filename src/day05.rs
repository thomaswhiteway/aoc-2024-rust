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

fn follow(start: u32, values: &[u32], links: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    let mut chained = HashSet::new();

    let mut to_check = vec![start];

    while let Some(check) = to_check.pop() {
        if let Some(candidates) = links.get(&check) {
            for &val in values {
                if candidates.contains(&val) && chained.insert(val) {
                    to_check.push(val);
                }
            }
        }
    }

    chained.into_iter().collect()
}

#[derive(Clone)]
pub struct Rules {
    before: HashMap<u32, HashSet<u32>>,
    after: HashMap<u32, HashSet<u32>>,
}

impl Rules {
    fn new(rules: Vec<(u32, u32)>) -> Self {
        let mut after: HashMap<u32, HashSet<u32>> = HashMap::new();
        let mut before: HashMap<u32, HashSet<u32>> = HashMap::new();

        for (first, second) in rules {
            after.entry(first).or_default().insert(second);
            before.entry(second).or_default().insert(first);
        }

        Rules { before, after }
    }

    fn in_sequence(&self, page_numbers: &[u32]) -> bool {
        let mut disallowed: HashSet<u32> = HashSet::new();

        for page in page_numbers {
            if disallowed.contains(page) {
                return false;
            }

            if let Some(before) = self.before.get(page) {
                disallowed.extend(before);
            }
        }

        true
    }

    fn reorder(&self, page_numbers: &[u32]) -> Box<[u32]> {
        if let Some(&first) = page_numbers.first() {
            let before = follow(first, page_numbers, &self.before);
            let after = follow(first, page_numbers, &self.after);

            let before = self.reorder(&before);
            let after = self.reorder(&after);

            assert!(before.len() + after.len() + 1 == page_numbers.len());

            let mut new_page_numbers = vec![];
            new_page_numbers.extend(before);
            new_page_numbers.push(first);
            new_page_numbers.extend(after);
            new_page_numbers.into_boxed_slice()
        } else {
            vec![].into_boxed_slice()
        }
    }
}

fn find_mid_number(page_numbers: &[u32]) -> u32 {
    page_numbers[page_numbers.len() / 2]
}

fn find_ordered_mid_numbers(rules: &Rules, sequences: &[Box<[u32]>]) -> u32 {
    sequences
        .iter()
        .filter(|page_numbers| rules.in_sequence(page_numbers))
        .map(|page_numbers| find_mid_number(page_numbers))
        .sum()
}

fn find_unordered_mid_numbers(rules: &Rules, sequences: &[Box<[u32]>]) -> u32 {
    sequences
        .iter()
        .filter(|page_numbers| !rules.in_sequence(page_numbers))
        .map(|page_numbers| rules.reorder(page_numbers))
        .map(|page_numbers| find_mid_number(&page_numbers))
        .sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = (Rules, Box<[Box<[u32]>]>);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        parse_input(&data)
    }

    fn solve((rules, page_numbers): Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_ordered_mid_numbers(&rules, &page_numbers);
        let part2 = find_unordered_mid_numbers(&rules, &page_numbers);

        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
