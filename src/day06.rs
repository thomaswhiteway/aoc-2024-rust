use std::collections::HashSet;

use failure::Error;

use crate::common::{Direction, Position};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
    fn with_obstacle_at(&self, position: Position) -> Self {
        let mut obstacles = self.obstacles.clone();
        obstacles.insert(position);
        Grid {
            obstacles,
            width: self.width,
            height: self.height,
        }
    }

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

fn find_guard_locations(grid: &Grid, guard: Guard) -> impl Iterator<Item = Guard> + '_ {
    [guard].into_iter().chain((0..).scan(guard, |guard, _| {
        if guard.do_move(grid) {
            Some(*guard)
        } else {
            None
        }
    }))
}

fn find_visited_positions(grid: &Grid, guard: Guard) -> HashSet<Position> {
    find_guard_locations(grid, guard)
        .map(|guard| guard.position)
        .collect()
}

fn path_loops(grid: &Grid, mut guard: Guard) -> bool {
    let mut visited = HashSet::new();
    visited.insert(guard);

    while guard.do_move(grid) {
        if !visited.insert(guard) {
            return true;
        }
    }

    false
}

fn count_loop_locations(
    grid: &Grid,
    visited_positions: &HashSet<Position>,
    guard_start: Guard,
) -> usize {
    visited_positions
        .iter()
        .filter(|&&position| position != guard_start.position)
        .filter(|&&position| {
            let new_grid = grid.with_obstacle_at(position);
            path_loops(&new_grid, guard_start)
        })
        .count()
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
        let visited_positions = find_visited_positions(&grid, guard);
        let part1 = visited_positions.len();
        let part2 = count_loop_locations(&grid, &visited_positions, guard);

        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
