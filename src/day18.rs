use std::collections::HashSet;

use failure::{err_msg, Error};
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::{all_consuming, map},
    multi::many1,
    sequence::{separated_pair, terminated},
};

use crate::{common::Position, parsers::unsigned};

#[derive(Debug)]
struct Grid {
    occupied: HashSet<Position>,
    max_pos: Position,
}

impl Grid {
    fn new<I: IntoIterator<Item = Position>>(bytes: I, max_pos: Position) -> Self {
        Grid {
            occupied: bytes.into_iter().collect(),
            max_pos,
        }
    }

    fn can_move_to(&self, pos: Position) -> bool {
        (0..=self.max_pos.x).contains(&pos.x)
            && (0..=self.max_pos.y).contains(&pos.y)
            && !self.occupied.contains(&pos)
    }
}

fn find_min_steps(num_bytes: usize, max_pos: Position, bytes: &[Position]) -> Option<u64> {
    let grid = Grid::new(bytes.iter().take(num_bytes).cloned(), max_pos);

    let mut visited = HashSet::new();
    let mut current = HashSet::new();
    current.insert(Position::origin());

    for steps in 0.. {
        if current.contains(&max_pos) {
            return Some(steps);
        } else if current.is_empty() {
            return None;
        }

        let new_current = current
            .iter()
            .flat_map(|pos| pos.adjacent())
            .filter(|pos| grid.can_move_to(*pos))
            .filter(|pos| !visited.contains(pos))
            .collect();
        visited.extend(current);
        current = new_current;
    }

    unreachable!()
}

fn find_first_blocker(max_pos: Position, bytes: &[Position]) -> Position {
    let mut num_bytes = 1024;

    while find_min_steps(num_bytes, max_pos, bytes).is_some() {
        println!("First {} bytes still works", num_bytes);
        num_bytes *= 2;
    }

    let mut lower = num_bytes / 2;
    let mut upper = num_bytes;

    while upper > lower + 1 {
        let mid = (upper + lower) / 2;
        println!("Checking {} - {} - {}", lower, mid, upper);

        if find_min_steps(mid, max_pos, bytes).is_some() {
            lower = mid;
        } else {
            upper = mid;
        }
    }

    bytes[lower]
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Box<[Position]>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let byte = map(separated_pair(unsigned, tag(","), unsigned), |(x, y)| {
            Position { x, y }
        });
        let bytes = map(many1(terminated(byte, newline)), Vec::into_boxed_slice);

        all_consuming(bytes)(&data)
            .map(|(_, bytes)| bytes)
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
    }

    fn solve(bytes: Self::Problem) -> (Option<String>, Option<String>) {
        let max_pos = Position { x: 70, y: 70 };
        let part1 = find_min_steps(1024, max_pos, &bytes).unwrap();
        let part2 = find_first_blocker(max_pos, &bytes);
        (
            Some(part1.to_string()),
            Some(format!("{},{}", part2.x, part2.y)),
        )
    }
}
