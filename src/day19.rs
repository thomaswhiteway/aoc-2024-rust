use std::collections::{HashMap, HashSet};

use failure::Error;

fn num_arrangements(pattern: &str, towels: &[String]) -> usize {
    let mut arrangements = HashMap::new();
    arrangements.insert("", 1);

    for index in (0..pattern.len()).rev() {
        let remaining = &pattern[index..];

        let mut num = 0;

        for towel in towels {
            if remaining.starts_with(towel) {
                let next = &remaining[towel.len()..];
                num += arrangements[next];
            }
        }

        arrangements.insert(remaining, num);
    }

    arrangements[pattern]
}

fn is_possible(pattern: &str, towels: &[String]) -> bool {
    let mut stack = vec![(pattern)];

    let mut added = HashSet::new();
    added.insert(pattern);

    while let Some(remaining) = stack.pop() {
        if remaining.is_empty() {
            return true;
        }

        for towel in towels {
            if remaining.starts_with(towel) {
                let next: &str = &remaining[towel.len()..];
                if !added.contains(next) {
                    added.insert(next);
                    stack.push(next);
                }
            }
        }
    }

    false
}

fn find_num_possible(towels: &[String], patterns: &[String]) -> usize {
    patterns
        .iter()
        .filter(|pattern| is_possible(pattern, towels))
        .count()
}

fn find_num_arrangements(towels: &[String], patterns: &[String]) -> usize {
    patterns
        .iter()
        .map(|pattern| num_arrangements(pattern, towels))
        .sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = (Box<[String]>, Box<[String]>);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let lines: Vec<_> = data.lines().collect();

        let towels = lines[0]
            .split(", ")
            .map(str::to_string)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let patterns = lines[2..]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Ok((towels, patterns))
    }

    fn solve((towels, patterns): Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_num_possible(&towels, &patterns);
        let part2 = find_num_arrangements(&towels, &patterns);
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
