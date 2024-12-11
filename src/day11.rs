use std::collections::HashMap;

use failure::{err_msg, Error};
use itertools::Either;

fn updated_stones(stone: u64) -> impl Iterator<Item = u64> {
    if stone == 0 {
        Either::Left([1].into_iter())
    } else {
        let width = stone.ilog10() + 1;
        if width % 2 == 0 {
            let base = 10u64.pow(width / 2);
            Either::Right([stone / base, stone % base].into_iter())
        } else {
            Either::Left([stone * 2024].into_iter())
        }
    }
}

fn count_stones(init_stones: Vec<u64>, blinks: usize) -> usize {
    let mut stones = HashMap::new();
    for stone in init_stones {
        *stones.entry(stone).or_default() += 1;
    }

    for _ in 0..blinks {
        let mut new_stones = HashMap::new();

        for (stone, count) in stones {
            for new_stone in updated_stones(stone) {
                *new_stones.entry(new_stone).or_default() += count;
            }
        }

        stones = new_stones;
    }

    stones.values().sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Vec<u64>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        data.split_whitespace()
            .map(|val| {
                val.parse()
                    .map_err(|_| err_msg(format!("Invalid value {}", val)))
            })
            .collect()
    }

    fn solve(stones: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = count_stones(stones.clone(), 25);
        let part2 = count_stones(stones, 75);
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
