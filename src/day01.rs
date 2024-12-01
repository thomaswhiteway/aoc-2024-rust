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

fn solve(mut left: Vec<u32>, mut right: Vec<u32>) -> u32 {
    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
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
        let part1: u32 = solve(left, right);

        (Some(part1.to_string()), None)
    }
}
