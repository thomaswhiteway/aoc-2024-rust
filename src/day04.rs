use std::collections::HashMap;

use failure::Error;

use crate::common::{Direction, Position};
use itertools::iproduct;

pub struct Solver {}

fn has_match(grid: &HashMap<Position, char>, start: Position, dir: Direction) -> bool {
    let candidate: String = (0..4)
        .map(|offset| start.step_by(dir, offset))
        .map(|pos| grid.get(&pos).cloned().unwrap_or(' '))
        .collect();

    candidate == "XMAS"
}

fn find_num_occurances(max: Position, grid: &HashMap<Position, char>) -> usize {
    iproduct!(0..=max.x, 0..=max.y, Direction::all())
        .map(|(x, y, dir)| (Position { x, y }, dir))
        .filter(|&(pos, dir)| has_match(grid, pos, dir))
        .count()
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
        let part1 = find_num_occurances(max, &grid);

        (Some(part1.to_string()), None)
    }
}
