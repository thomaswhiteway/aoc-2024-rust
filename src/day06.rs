use std::collections::HashSet;

use failure::Error;

use crate::common::{Direction, Position};

#[derive(Debug, Clone, Copy)]
pub struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn do_move(&mut self, grid: &Grid) -> bool {
        let position = self.position.step(self.direction);

        if grid.is_occupied(position) {
            self.direction = self.direction.turn_right();
        } else {
            self.position = position
        }

        grid.inside(self.position)
    }
}

pub struct Grid {
    obstacles: HashSet<Position>,
    width: i64,
    height: i64,
}

impl Grid {
    fn is_occupied(&self, position: Position) -> bool {
        self.obstacles.contains(&position)
    }

    fn inside(&self, position: Position) -> bool {
        (0..self.width).contains(&position.x) && (0..self.height).contains(&position.y)
    }
}

fn find_positions<'a>(rows: &'a [&'a str], c: char) -> impl Iterator<Item = Position> + 'a {
    rows.iter().enumerate().flat_map(move |(y, row)| {
        row.char_indices()
            .filter_map(move |(x, c1)| if c1 == c { Some((x, y).into()) } else { None })
    })
}

fn count_visited_positions(grid: &Grid, mut guard: Guard) -> usize {
    let mut visited = HashSet::new();
    visited.insert(guard.position);

    while guard.do_move(grid) {
        visited.insert(guard.position);
    }

    visited.len()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = (Grid, Guard);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let lines: Vec<_> = data.lines().collect();

        let height = lines.len() as i64;
        let width = lines[0].len() as i64;

        let obstacles = find_positions(&lines, '#').collect();
        let guard = find_positions(&lines, '^')
            .map(|position| Guard {
                position,
                direction: Direction::North,
            })
            .next()
            .unwrap();

        Ok((
            Grid {
                obstacles,
                height,
                width,
            },
            guard,
        ))
    }

    fn solve((grid, guard): Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = count_visited_positions(&grid, guard);

        (Some(part1.to_string()), None)
    }
}
