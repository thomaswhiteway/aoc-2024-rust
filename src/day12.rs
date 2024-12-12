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

    fn sides(&self) -> usize {
        let mut to_visit: HashSet<Position> = self
            .plots
            .iter()
            .flat_map(|pos| pos.adjacent())
            .collect::<HashSet<_>>()
            .difference(&self.plots)
            .cloned()
            .collect();

        let mut sides = 0;

        while let Some(start) = to_visit.iter().next().cloned() {
            let start_dir = Direction::cardinal()
                .find(|&dir| self.plots.contains(&start.step(dir)))
                .unwrap()
                .turn_left();

            let mut pos = start;
            let mut dir = start_dir;

            loop {
                to_visit.remove(&pos);

                let next_pos = pos.step(dir);

                if self.contains(next_pos) {
                    sides += 1;
                    dir = dir.turn_left();
                } else if !self.contains(next_pos.step(dir.turn_right())) {
                    sides += 1;
                    dir = dir.turn_right();
                    pos = next_pos.step(dir);
                } else {
                    pos = next_pos;
                }

                if pos == start && dir == start_dir {
                    break;
                }
            }
        }

        sides
    }

    fn full_price(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn discounted_price(&self) -> usize {
        self.area() * self.sides()
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

fn get_total_price<F>(plots: &HashMap<Position, char>, price: F) -> usize
where
    F: Fn(&Region) -> usize,
{
    find_regions(plots).iter().map(price).sum()
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
        let part1 = get_total_price(&plots, Region::full_price);
        let part2 = get_total_price(&plots, Region::discounted_price);
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
