use crate::common::count_occurances;
use failure::{err_msg, Error};

pub struct Solver {}

fn parse_line(line: &str) -> Result<(u32, u32), Error> {
    let nums: Vec<_> = line
        .split_whitespace()
        .map(|num| num.parse())
        .collect::<Result<_, _>>()?;
    if let [left, right] = nums[..] {
        Ok((left, right))
    } else {
        Err(err_msg("Incorrect number of numbers"))
    }
}

fn count_difference(mut left: Vec<u32>, mut right: Vec<u32>) -> u32 {
    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

fn count_similarity(left: &[u32], right: &[u32]) -> u32 {
    let counts = count_occurances(right);

    left.iter()
        .map(|num| *num * counts.get(num).cloned().unwrap_or_default())
        .sum()
}

impl super::Solver for Solver {
    type Problem = (Vec<u32>, Vec<u32>);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        data.lines()
            .map(parse_line)
            .collect::<Result<Vec<_>, _>>()
            .map(|rows| rows.into_iter().unzip())
    }

    fn solve((left, right): Self::Problem) -> (Option<String>, Option<String>) {
        let part1: u32 = count_difference(left.clone(), right.clone());
        let part2: u32 = count_similarity(&left, &right);

        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
