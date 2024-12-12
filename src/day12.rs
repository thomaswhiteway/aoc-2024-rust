use crate::common::{Direction, Position};
use failure::Error;
use itertools::iproduct;
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug, Clone)]
struct Region {
    plots: HashSet<Position>,
}

impl Region {
    fn add(&mut self, position: Position) {
        self.plots.insert(position);
    }

    fn contains(&self, position: Position) -> bool {
        self.plots.contains(&position)
    }

    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        iproduct!(self.plots.iter(), Direction::cardinal())
            .filter(|&(pos, dir)| !self.contains(pos.step(dir)))
            .count()
    }

    fn price(&self) -> usize {
        self.area() * self.perimeter()
    }
}

fn find_region(plots: &HashMap<Position, char>, position: Position) -> Region {
    let mut region = Region::default();
    let plant = plots.get(&position);

    let mut checked = HashSet::new();

    let mut to_check = HashSet::new();
    to_check.insert(position);

    while let Some(pos) = to_check.iter().cloned().next() {
        to_check.remove(&pos);
        checked.insert(pos);

        if plots.get(&pos) == plant {
            region.add(pos);

            for next_pos in pos.adjacent() {
                if !checked.contains(&next_pos) {
                    to_check.insert(next_pos);
                }
            }
        }
    }

    region
}

fn find_regions(plots: &HashMap<Position, char>) -> Vec<Region> {
    let mut remaining: HashSet<Position> = plots.keys().cloned().collect();
    let mut regions = vec![];

    while let Some(position) = remaining.iter().cloned().next() {
        let region = find_region(plots, position);
        remaining.retain(|&pos| !region.contains(pos));
        regions.push(region);
    }

    regions
}

fn get_total_price(plots: &HashMap<Position, char>) -> usize {
    find_regions(plots).iter().map(Region::price).sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = HashMap<Position, char>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        Ok(data
            .lines()
            .enumerate()
            .flat_map(|(y, row)| row.char_indices().map(move |(x, c)| ((x, y).into(), c)))
            .collect())
    }

    fn solve(plots: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = get_total_price(&plots);
        (Some(part1.to_string()), None)
    }
}
