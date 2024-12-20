#![allow(unused)]
use priority_queue::PriorityQueue;
use std::{collections::{HashSet, HashMap}, fmt::Debug, hash::Hash};

pub trait State: Sized + Eq + PartialEq + Hash {
    fn heuristic(&self) -> u64;
    fn successors(&self) -> Vec<(u64, Self)>;
    fn is_end(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Priority(u64);

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

#[derive(Clone)]
pub struct Solution<S> {
    pub cost: u64,
    pub route: Vec<S>,
}

impl<S: Clone> Solution<S> {
    fn new(state: S) -> Self {
        Solution {
            cost: 0,
            route: vec![state]
        }
    }

    fn successor(&self, state: S, delta: u64) -> Self {
        let mut route = self.route.clone();
        route.push(state.clone());
        Solution {
            cost: self.cost + delta,
            route,
        }
    }
}

impl<S: State> Solution<S> {
    fn priority(&self) -> Priority {
        Priority(self.cost + self.route.last().unwrap().heuristic())
    }

}

impl<S: State> PartialEq for Solution<S> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl<S: State> Eq for Solution<S> {
}


impl<S: State> PartialOrd for Solution<S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: State> Ord for Solution<S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority().cmp(&other.priority())
    }
}

pub fn solve<S: State + Clone + Debug, I: IntoIterator<Item=S>>(
    starts: I,
) -> Result<Solution<S>, HashSet<S>>
{
    let mut queue = PriorityQueue::new();
    for start in starts {
        let solution = Solution::new(start.clone());
        queue.push(start, solution);
    }

    let mut visited = HashSet::new();

    while let Some((state, solution)) = queue.pop() {
        if state.is_end() {
            return Ok(solution);
        }

        visited.insert(state.clone());

        for (delta, next_state) in state.successors() {
            if visited.contains(&next_state) {
                continue;
            }

            queue.push_increase(next_state.clone(), solution.successor(next_state, delta));
         }
    }

    Err(visited)
}
