use std::collections::HashSet;

use failure::Error;

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
                let next = &remaining[towel.len()..];
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
    patterns.iter().filter(|pattern| is_possible(pattern, towels)).count()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = (Box<[String]>, Box<[String]>);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let lines: Vec<_> = data.lines().collect();

        let towels = lines[0].split(", ").map(str::to_string).collect::<Vec<_>>().into_boxed_slice();
        let patterns =lines[2..].iter().map(|s| s.to_string()).collect::<Vec<_>>().into_boxed_slice();

        Ok((towels, patterns))
    }

    fn solve((towels, patterns): Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_num_possible(&towels, &patterns);
        (Some(part1.to_string()), None)
    }
}
