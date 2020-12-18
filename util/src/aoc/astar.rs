use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

type Cost = u32;

#[derive(Eq, PartialEq, Debug)]
pub struct AStarResult<T: Eq + Hash> {
    pub failed: bool,
    pub path: Vec<T>,
}

impl<T: Clone + Eq + Hash> AStarResult<T> {
    fn from_path(start: T, path: HashMap<T, T>) -> AStarResult<T> {
        let mut points = Vec::new();
        points.push(start.to_owned());

        let mut point = &start;
        while let Some(next_point) = path.get(&point) {
            let owned_point = next_point.to_owned();
            points.push(owned_point);
            point = next_point;
        }

        points.reverse();

        AStarResult {
            failed: false,
            path: points,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct ValueWithCost<T>
where
    T: Eq,
{
    value: T,
    cost: Cost,
}

impl<T: Eq> Ord for ValueWithCost<T> {
    fn cmp(&self, other: &ValueWithCost<T>) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<T: Eq> PartialOrd for ValueWithCost<T> {
    fn partial_cmp(&self, other: &ValueWithCost<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn search<T: Clone + Debug + Hash + Eq, HF: Fn(&T) -> Cost, MF: Fn(&T) -> Vec<T>>(
    start: &T,
    h_fn: HF,
    move_fn: MF,
) -> AStarResult<T> {
    let mut frontier: BinaryHeap<ValueWithCost<T>> = BinaryHeap::new();
    frontier.push(ValueWithCost {
        value: start.to_owned(),
        cost: h_fn(&start),
    });
    let mut previous: HashMap<T, T> = HashMap::new();
    let mut path_cost: HashMap<T, Cost> = HashMap::new();
    path_cost.insert(start.to_owned(), 0);

    while let Some(ValueWithCost { value, cost: _ }) = frontier.pop() {
        if h_fn(&value) == 0 {
            return AStarResult::from_path(value, previous);
        }

        let new_cost = get_cost(&value, &path_cost).unwrap() + 1;
        for new_value in move_fn(&value).into_iter() {
            let current_cost = get_cost(&new_value, &path_cost);

            if current_cost.is_none() || new_cost < current_cost.unwrap() {
                let nv = new_value.to_owned();

                frontier.push(ValueWithCost {
                    value: nv.to_owned(),
                    cost: new_cost + h_fn(&new_value),
                });

                path_cost.insert(nv.to_owned(), new_cost);
                previous.insert(nv.to_owned(), value.to_owned());
            }
        }
    }

    AStarResult {
        failed: true,
        path: vec![],
    }
}

fn get_cost<T: Clone + Hash + Eq>(value: &T, cost_map: &HashMap<T, Cost>) -> Option<u32> {
    match cost_map.get(value) {
        Some(cost) => Some(cost.clone()),
        _ => None,
    }
}
