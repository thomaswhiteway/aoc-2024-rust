use std::collections::{HashMap, HashSet};

use failure::{err_msg, Error};
use itertools::iproduct;

use crate::common::{find_all_symbols_in_grid, find_symbol_in_grid, Position};

fn find_distances_from(pos: Position, walls: &HashSet<Position>) -> HashMap<Position, u64> {
    let mut distances = HashMap::new();

    let mut current = HashSet::new();
    current.insert(pos);

    for picoseconds in 0.. {
        if current.is_empty() {
            break;
        }

        for pos in current.iter() {
            distances.insert(*pos, picoseconds);
        }

        current = current
            .iter()
            .flat_map(|pos| pos.adjacent())
            .filter(|pos| !walls.contains(pos))
            .filter(|pos| !distances.contains_key(pos))
            .collect();
    }

    distances
}

fn find_cheats_better_than(
    start: Position,
    end: Position,
    walls: &HashSet<Position>,
    min_improvement: u64,
) -> usize {
    let from_start = find_distances_from(start, walls);
    let from_end = find_distances_from(end, walls);

    let best = *from_start.get(&end).unwrap();
    walls
        .iter()
        .filter(|&wall| {
            iproduct!(
                wall.adjacent().filter_map(|pos| from_start.get(&pos)),
                wall.adjacent().filter_map(|pos| from_end.get(&pos))
            )
            .map(|(t1, t2)| t1 + t2 + 2)
            .min()
            .map(|t| t + min_improvement <= best)
            .unwrap_or_default()
        })
        .count()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = (Position, Position, HashSet<Position>);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let start =
            find_symbol_in_grid(&data, 'S').ok_or(err_msg("Failed to find start position"))?;
        let end = find_symbol_in_grid(&data, 'E').ok_or(err_msg("Failed to find end position"))?;
        let walls = find_all_symbols_in_grid(&data, '#').collect();

        Ok((start, end, walls))
    }

    fn solve((start, end, walls): Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_cheats_better_than(start, end, &walls, 100);
        (Some(part1.to_string()), None)
    }
}
