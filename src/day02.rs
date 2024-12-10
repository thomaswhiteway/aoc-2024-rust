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
    diffs.iter().all(|&d| (-3..=3).contains(&d) && d != 0)
        && (diffs.iter().all(|&d| d > 0) || diffs.iter().all(|&d| d < 0))
}

fn is_safe_damped(levels: &[i64]) -> bool {
    is_safe(levels)
        || (0..levels.len()).any(|index| {
            let mut new_levels = levels.to_vec();
            new_levels.remove(index);
            is_safe(&new_levels)
        })
}

fn count_safe<F: Fn(&[i64]) -> bool>(all_levels: &[Box<[i64]>], is_safe: F) -> usize {
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
        let part1 = count_safe(&levels, is_safe);
        let part2 = count_safe(&levels, is_safe_damped);

        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
