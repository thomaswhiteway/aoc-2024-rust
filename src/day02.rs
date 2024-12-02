use failure::{err_msg, Error};
use itertools::Itertools;

pub struct Solver {}

fn parse_line(line: &str) -> Result<Box<[i64]>, Error> {
    line.split_whitespace()
        .map(|num| {
            num.parse()
                .map_err(|_| err_msg(format!("Invalid number {}", num)))
        })
        .collect()
}

fn is_safe(levels: &[i64]) -> bool {
    let diffs: Vec<i64> = levels.iter().tuple_windows().map(|(x, y)| x - y).collect();
    diffs.iter().all(|&d| d >= -3 && d <= 3 && d != 0)
        && (diffs.iter().all(|&d| d > 0) || diffs.iter().all(|&d| d < 0))
}

fn count_safe(all_levels: &[Box<[i64]>]) -> usize {
    all_levels.iter().filter(|levels| is_safe(levels)).count()
}

impl super::Solver for Solver {
    type Problem = Box<[Box<[i64]>]>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        data.lines()
            .map(parse_line)
            .collect::<Result<Vec<_>, _>>()
            .map(|levels| levels.into_boxed_slice())
    }

    fn solve(levels: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = count_safe(&levels);

        (Some(part1.to_string()), None)
    }
}
