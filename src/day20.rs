use std::collections::{HashMap, HashSet};

use failure::{err_msg, Error};

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
    cheat_ps: u64,
    min_improvement: u64,
) -> usize {
    let from_start = find_distances_from(start, walls);
    let from_end = find_distances_from(end, walls);
    let from_end_ref = &from_end;

    let best = *from_start.get(&end).unwrap();

    from_start
        .iter()
        .flat_map(|(cheat_start, t1)| {
            cheat_start
                .within_range(cheat_ps as i64)
                .filter_map(move |cheat_end| {
                    from_end_ref
                        .get(&cheat_end)
                        .map(|t2| t1 + t2 + cheat_start.manhattan_distance_to(&cheat_end))
                })
        })
        .filter(|t| t + min_improvement <= best)
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
        let part1 = find_cheats_better_than(start, end, &walls, 2, 100);
        let part2 = find_cheats_better_than(start, end, &walls, 20, 100);
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
