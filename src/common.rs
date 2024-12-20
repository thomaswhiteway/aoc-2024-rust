#![allow(unused)]

use failure::{err_msg, Error};
use itertools::iproduct;
use num::rational::Ratio;
use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn reflect_x(&self, x: i64) -> Position {
        Position {
            x: x + (x - self.x),
            y: self.y,
        }
    }

    pub fn gradient(&self) -> Ratio<i64> {
        Ratio::new(self.y, self.x)
    }

    pub fn manhattan_distance_to(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn adjacent(self) -> impl Iterator<Item = Position> + Clone {
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .into_iter()
            .map(move |(dx, dy)| Position {
                x: self.x + dx,
                y: self.y + dy,
            })
    }

    pub fn surrounding(&self) -> impl Iterator<Item = Position> + '_ {
        iproduct!([-1, 0, 1], [-1, 0, 1]).filter_map(|(dx, dy)| {
            if dx != 0 || dy != 0 {
                Some(Position {
                    x: self.x + dx,
                    y: self.y + dy,
                })
            } else {
                None
            }
        })
    }

    pub fn direction_to(&self, other: &Self) -> Option<Direction> {
        match (other.x - self.x, other.y - self.y) {
            (0, dy) if dy < 0 => Some(Direction::North),
            (dx, 0) if dx > 0 => Some(Direction::East),
            (0, dy) if dy > 0 => Some(Direction::South),
            (dx, 0) if dx < 0 => Some(Direction::West),
            _ => None,
        }
    }

    pub fn within_range(
        self,
        distance: i64,
    ) -> impl Iterator<Item = Position> {
        let min_y = self.y - distance;
        let max_y = self.y + distance;

        (min_y..=max_y).flat_map(move |y| {
            let rem = distance - (self.y - y).abs();

            let min_x = self.x - rem;
            let max_x = self.x + rem;

            (min_x..=max_x).map(move |x| Position { x, y })
        })
    }

    pub fn length(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    pub fn points_to(self, other: Position) -> impl Iterator<Item = Position> {
        let diff = other - self;
        assert!(diff.x == 0 || diff.y == 0);
        let distance = diff.length();
        let delta = diff / distance;
        (0..distance).map(move |index| self + delta * index)
    }

    pub fn step(self, direction: Direction) -> Self {
        self + direction.offset()
    }

    pub fn step_by(self, direction: Direction, len: u32) -> Self {
        self + direction.offset() * len as i64
    }

    pub fn origin() -> Self {
        Position { x: 0, y: 0 }
    }
}

impl From<(i64, i64)> for Position {
    fn from((x, y): (i64, i64)) -> Self {
        Position { x, y }
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Position {
            x: x as i64,
            y: y as i64,
        }
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div<i64> for Position {
    type Output = Self;
    fn div(self, rhs: i64) -> Self::Output {
        Position {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Mul<i64> for Position {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Position {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn all() -> impl Iterator<Item = Self> + Clone {
        use Direction::*;
        [
            North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
        ]
        .into_iter()
    }

    pub fn cardinal() -> impl Iterator<Item = Self> + Clone {
        use Direction::*;
        [North, East, South, West].into_iter()
    }

    pub fn diagonal() -> impl Iterator<Item = Self> + Clone {
        use Direction::*;
        [NorthEast, SouthEast, SouthWest, NorthWest].into_iter()
    }

    pub fn as_char(&self) -> char {
        use Direction::*;
        match self {
            North => '^',
            NorthEast => 'p',
            East => '>',
            SouthEast => 'b',
            South => 'V',
            SouthWest => 'L',
            West => '<',
            NorthWest => '\\',
        }
    }

    pub fn offset(self) -> Position {
        use Direction::*;
        match self {
            North => Position { x: 0, y: -1 },
            NorthEast => Position { x: 1, y: -1 },
            East => Position { x: 1, y: 0 },
            SouthEast => Position { x: 1, y: 1 },
            South => Position { x: 0, y: 1 },
            SouthWest => Position { x: -1, y: 1 },
            West => Position { x: -1, y: 0 },
            NorthWest => Position { x: -1, y: -1 },
        }
    }

    pub fn reverse(self) -> Direction {
        use Direction::*;
        match self {
            North => South,
            NorthEast => SouthWest,
            East => West,
            SouthEast => NorthWest,
            South => North,
            SouthWest => NorthEast,
            West => East,
            NorthWest => SouthEast,
        }
    }

    pub fn turn_left(self) -> Direction {
        use Direction::*;
        match self {
            North => West,
            NorthEast => NorthWest,
            East => North,
            SouthEast => NorthEast,
            South => East,
            SouthWest => SouthEast,
            West => South,
            NorthWest => SouthWest,
        }
    }

    pub fn turn_right(self) -> Direction {
        use Direction::*;
        match self {
            North => East,
            NorthEast => SouthEast,
            East => South,
            SouthEast => SouthWest,
            South => West,
            SouthWest => NorthWest,
            West => North,
            NorthWest => NorthEast,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::North),
            '>' => Ok(Direction::East),
            'v' => Ok(Direction::South),
            '<' => Ok(Direction::West),
            c => Err(err_msg(format!("Invalid move {}", c))),
        }
    }
}

pub fn count_occurances<T: Hash + Eq + Clone, I: IntoIterator<Item = T>>(
    items: I,
) -> HashMap<T, u32> {
    let mut counts = HashMap::new();

    for item in items {
        *counts.entry(item.clone()).or_default() += 1;
    }

    counts
}

pub fn find_symbol_in_grid(data: &str, symbol: char) -> Option<Position> {
    data.lines().enumerate().find_map(|(y, line)| {
        line.char_indices().find_map(|(x, c)| {
            if c == symbol {
                Some((x, y).into())
            } else {
                None
            }
        })
    })
}

pub fn find_all_symbols_in_grid(data: &str, symbol: char) -> impl Iterator<Item = Position> + '_ {
    data.lines().enumerate().flat_map(move |(y, line)| {
        line.char_indices().filter_map(move |(x, c)| {
            if c == symbol {
                Some((x, y).into())
            } else {
                None
            }
        })
    })
}
