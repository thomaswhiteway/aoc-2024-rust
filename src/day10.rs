use std::{collections::HashSet, str::FromStr};

use crate::common::Position;
use failure::{err_msg, Error};

pub struct Map {
    heights: Vec<Vec<u8>>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .map(|d| d as u8)
                            .ok_or_else(|| err_msg(format!("Invalid digit: {}", c)))
                    })
                    .collect()
            })
            .collect::<Result<_, _>>()
            .map(|heights| Map { heights })
    }
}

impl Map {
    fn all_positions(&self) -> impl Iterator<Item = Position> + '_ {
        self.heights
            .iter()
            .enumerate()
            .flat_map(|(y, row)| (0..row.len()).map(move |x| (x, y).into()))
    }

    fn height_at(&self, position: Position) -> Option<u8> {
        self.heights
            .get(position.y as usize)
            .and_then(|row| row.get(position.x as usize).cloned())
    }

    fn trailhead_score(&self, position: Position) -> usize {
        if self.height_at(position) != Some(0) {
            return 0;
        }

        let mut reachable = HashSet::new();
        reachable.insert(position);

        for height in 1..=9 {
            reachable = reachable
                .into_iter()
                .flat_map(|pos| pos.adjacent())
                .filter(|&pos| self.height_at(pos) == Some(height))
                .collect();
        }

        reachable.len()
    }
}

fn total_trailhead_score(map: &Map) -> usize {
    map.all_positions()
        .map(|pos| map.trailhead_score(pos))
        .sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Map;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        data.parse()
    }

    fn solve(map: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = total_trailhead_score(&map);
        (Some(part1.to_string()), None)
    }
}
