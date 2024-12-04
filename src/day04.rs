use std::collections::HashMap;

use failure::Error;

use crate::common::{count_occurances, Direction, Position};
use itertools::iproduct;

pub struct Solver {}

fn has_match(grid: &HashMap<Position, char>, start: Position, dir: Direction, value: &str) -> bool {
    value
        .char_indices()
        .map(|(offset, c)| (start.step_by(dir, offset as u32), c))
        .all(|(pos, c)| grid.get(&pos).cloned().unwrap_or('\0') == c)
}

fn find_xmas_count(max: Position, grid: &HashMap<Position, char>) -> usize {
    iproduct!(0..=max.x, 0..=max.y, Direction::all())
        .map(|(x, y, dir)| (Position { x, y }, dir))
        .filter(|&(pos, dir)| has_match(grid, pos, dir, "XMAS"))
        .count()
}

fn find_x_mas_count(max: Position, grid: &HashMap<Position, char>) -> usize {
    let middle_counts = count_occurances(
        iproduct!(0..=max.x, 0..=max.y, Direction::diagonal())
            .map(|(x, y, dir)| (Position { x, y }, dir))
            .filter(|&(pos, dir)| has_match(grid, pos, dir, "MAS"))
            .map(|(pos, dir)| pos.step(dir)),
    );

    middle_counts.values().filter(|&&count| count == 2).count()
}

impl super::Solver for Solver {
    type Problem = (Position, HashMap<Position, char>);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let grid: HashMap<Position, char> = data
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.char_indices().map(move |(x, c)| {
                    (
                        Position {
                            x: x as i64,
                            y: y as i64,
                        },
                        c,
                    )
                })
            })
            .collect();

        Ok((
            Position {
                x: grid.keys().map(|pos| pos.x).max().unwrap(),
                y: grid.keys().map(|pos| pos.y).max().unwrap(),
            },
            grid,
        ))
    }

    fn solve((max, grid): Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_xmas_count(max, &grid);
        let part2 = find_x_mas_count(max, &grid);

        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
