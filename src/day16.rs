use crate::{
    a_star,
    common::{find_all_symbols_in_grid, find_symbol_in_grid, Direction, Position},
};
use ansi_term::Colour;
use failure::{err_msg, Error};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

#[derive(Clone)]
struct State<'a> {
    position: Position,
    direction: Direction,
    end: Position,
    walls: &'a HashSet<Position>,
}

impl Debug for State<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "State {{ position: {}, direction: {:?} }}",
            self.position, self.direction
        )
    }
}

impl<'a> State<'a> {
    fn forward(&self) -> Option<Self> {
        let position = self.position.step(self.direction);
        if !self.walls.contains(&position) {
            Some(State { position, ..*self })
        } else {
            None
        }
    }

    fn turn_left(&self) -> Self {
        State {
            direction: self.direction.turn_left(),
            ..*self
        }
    }

    fn turn_right(&self) -> Self {
        State {
            direction: self.direction.turn_right(),
            ..*self
        }
    }
}

impl PartialEq for State<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.direction == other.direction
    }
}

impl Eq for State<'_> {}

impl Hash for State<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.direction.hash(state);
    }
}

impl<'a> a_star::State for State<'a> {
    fn is_end(&self) -> bool {
        self.position == self.end
    }

    fn heuristic(&self) -> u64 {
        0 //self.position.manhattan_distance_to(&self.end)
    }

    fn successors(&self) -> Vec<(u64, Self)> {
        let mut successors = vec![(1000, self.turn_left()), (1000, self.turn_right())];

        if let Some(forward) = self.forward() {
            successors.push((1, forward));
        }

        successors
    }
}

fn display_route(
    start: Position,
    end: Position,
    walls: &HashSet<Position>,
    route: &Vec<(Position, Direction)>,
) {
    let route_tiles: HashMap<_, _> = route.into_iter().cloned().collect();

    let max_x = walls.iter().map(|pos| pos.x).max().unwrap();
    let max_y = walls.iter().map(|pos| pos.y).max().unwrap();

    for y in 0..=max_y {
        print!("{:03}: ", y);
        for x in 0..=max_x {
            let pos = Position { x, y };

            let (colour, symbol) = if pos == start {
                (Colour::Blue.bold(), 'S')
            } else if pos == end {
                (Colour::Blue.bold(), 'E')
            } else if let Some(dir) = route_tiles.get(&pos) {
                (Colour::Green.bold(), dir.as_char())
            } else if walls.contains(&pos) {
                (Colour::Red.dimmed(), '#')
            } else {
                (Colour::White.dimmed(), '.')
            };

            print!("{}", colour.paint(symbol.to_string()));
        }

        println!();
    }
}

fn find_min_score(start: Position, end: Position, walls: &HashSet<Position>) -> u64 {
    let solution = a_star::solve([State {
        position: start,
        end,
        direction: Direction::East,
        walls,
    }])
    .unwrap();

    let route: Vec<(Position, Direction)> = solution
        .route
        .iter()
        .map(|state| (state.position, state.direction))
        .collect();

    display_route(start, end, walls, &route);

    solution.cost
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
        let part1 = find_min_score(start, end, &walls);
        (Some(part1.to_string()), None)
    }
}
