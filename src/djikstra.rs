#![allow(unused)]
use priority_queue::PriorityQueue;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

pub trait State: Sized + Eq + PartialEq + Hash {
    fn successors(&self) -> Vec<(u64, Self)>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cost(u64);

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

pub fn min_distance_from<S: State + Clone + Debug, I: IntoIterator<Item = S>>(
    starts: I,
) -> HashMap<S, u64> {
    let mut queue: PriorityQueue<S, _> = PriorityQueue::new();
    for start in starts {
        queue.push(start, Cost(0));
    }

    let mut visited = HashMap::new();

    while let Some((state, Cost(cost))) = queue.pop() {
        visited.insert(state.clone(), cost);

        for (delta, next_state) in state.successors() {
            if visited.contains_key(&next_state) {
                continue;
            }

            queue.push_increase(next_state, Cost(cost + delta));
        }
    }

    visited
}
