use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

type Cost = u32;

#[derive(Eq, PartialEq, Debug)]
pub enum AStarResult<T: Eq + Hash> {
    Success(Vec<T>, u32),
    Failed,
}

impl<T: Clone + Eq + Hash + Debug> AStarResult<T> {
    fn from_path(start: T, predecessor: &HashMap<T, T>, cost: &HashMap<T, Cost>) -> AStarResult<T> {
        let mut path = vec![start.clone()];
        let mut point = &start;
        while let Some(next_point) = predecessor.get(&point) {
            path.push(next_point.clone());
            point = next_point;
        }

        path.reverse();

        let total_cost = *cost.get(&path.iter().last().unwrap()).unwrap_or(&0);
        AStarResult::Success(path, total_cost)
    }
}

#[derive(Eq, PartialEq, Debug)]
struct ValueWithCost<T>
where
    T: Eq,
{
    value: T,
    estimated_cost: Cost,
}

impl<T: Eq> Ord for ValueWithCost<T> {
    fn cmp(&self, other: &ValueWithCost<T>) -> Ordering {
        other.estimated_cost.cmp(&self.estimated_cost)
    }
}

impl<T: Eq> PartialOrd for ValueWithCost<T> {
    fn partial_cmp(&self, other: &ValueWithCost<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn search<
    T: Clone + Debug + Hash + Eq,
    HF: Fn(&T) -> Cost,
    CF: Fn(&T, Option<&T>) -> Cost,
    MIT: IntoIterator<Item = T>,
    MF: Fn(&T) -> MIT,
>(
    start: &T,
    heuristic_fn: HF,
    cost_fn: CF,
    move_fn: MF,
) -> AStarResult<T> {
    let mut frontier: BinaryHeap<ValueWithCost<T>> = BinaryHeap::new();
    frontier.push(ValueWithCost {
        value: start.clone(),
        estimated_cost: heuristic_fn(&start),
    });
    let mut predecessor: HashMap<T, T> = HashMap::new();
    let mut path_cost: HashMap<T, Cost> = HashMap::new();
    path_cost.insert(start.clone(), cost_fn(start, None));

    while let Some(ValueWithCost {
        value: current_value,
        estimated_cost: _,
    }) = frontier.pop()
    {
        if heuristic_fn(&current_value) == 0 {
            return AStarResult::from_path(current_value, &predecessor, &path_cost);
        }

        for neighbor_value in move_fn(&current_value).into_iter() {
            let new_cost = get_path_cost(&current_value, &path_cost).unwrap()
                + cost_fn(&neighbor_value, Some(&current_value));
            let current_cost = get_path_cost(&neighbor_value, &path_cost);
            // Only re-explore a point if we haven't already visited it.
            if current_cost.is_none() {
                frontier.push(ValueWithCost {
                    value: neighbor_value.clone(),
                    estimated_cost: new_cost + heuristic_fn(&neighbor_value),
                });
            }

            if new_cost < current_cost.unwrap_or(u32::MAX) {
                path_cost.insert(neighbor_value.clone(), new_cost);
                predecessor.insert(neighbor_value, current_value.to_owned());
            }
        }
    }

    AStarResult::Failed
}

fn get_path_cost<T: Clone + Hash + Eq>(value: &T, cost_map: &HashMap<T, Cost>) -> Option<u32> {
    match cost_map.get(value) {
        Some(cost) => Some(cost.clone()),
        _ => None,
    }
}
