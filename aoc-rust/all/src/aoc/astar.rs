/**
@ToString
    @Getter
    @EqualsAndHashCode
    public static class AStarResult<T> {
        private final boolean failed;
        private final ImmutableList<T> path;

        private AStarResult() {
            this.failed = true;
            this.path = ImmutableList.of();
        }

        public AStarResult(Iterable<T> path) {
            this.failed = false;
            this.path = ImmutableList.copyOf(path);
        }

        public static <T> AStarResult<T> failed() {
            return new AStarResult<>();
        }
    }
    @Value
    public static class ValueWithCost<T> implements Comparable<ValueWithCost<T>> {
        T value;
        double cost;

        @Override
        public int compareTo(ValueWithCost<T> o) {
            return Double.compare(cost, o.cost);
        }

    }
    public static <T> AStarResult<T> fromPath(T p, Map<T, T> parentGraph) {
        Deque<T> points = new ArrayDeque<>();
        points.push(p);

        while(p != null) {
            p = parentGraph.get(p);

            if(p != null) {
                points.push(p);
            }
        }

        // Points were reversed by using Deque as a stack
        return new AStarResult<>(points);
    }

    public static <T> AStarResult<T> astarSearch(T start, Function<T, Double> hFunc, Function<T, List<T>> moveFunc) {
        PriorityQueue<ValueWithCost<T>> frontier = new PriorityQueue<>(
            Collections.singleton(new ValueWithCost<>(start, hFunc.apply(start)))
        );
        Map<T, T> previous = new HashMap<>();
        Map<T, Double> pathCost = new HashMap<>();
        pathCost.put(start, 0.0);
        previous.put(start, null);

        while(!frontier.isEmpty()) {
            ValueWithCost<T> p = frontier.poll();

            if(Math.abs(0 - hFunc.apply(p.value)) < 1e-6) {
                return fromPath(p.value, previous);
            }

            double newCost = pathCost.get(p.value) + 1;
            for(T newPoint : moveFunc.apply(p.value)) {
                Double cost = pathCost.get(newPoint);
                if(cost == null || newCost < cost) {
                    frontier.add(new ValueWithCost<>(newPoint, newCost + hFunc.apply(newPoint)));
                    pathCost.put(newPoint, newCost);
                    previous.put(newPoint, p.value);
                }
            }
        }

        return AStarResult.failed();
    }
*/
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

type Cost=u32;

#[derive(Eq, PartialEq, Debug)]
pub struct AStarResult<T: Eq + Hash> {
    pub failed: bool,
    pub path: Vec<T>
}

/*
    public static <T> AStarResult<T> fromPath(T p, Map<T, T> parentGraph) {
        Deque<T> points = new ArrayDeque<>();
        points.push(p);

        while(p != null) {
            p = parentGraph.get(p);

            if(p != null) {
                points.push(p);
            }
        }

        // Points were reversed by using Deque as a stack
        return new AStarResult<>(points);
    }
*/
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
            path: points
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct ValueWithCost<T>
        where T: Eq {
    value: T,
    cost: Cost
}

impl < T: Eq> Ord for ValueWithCost<T> {
    fn cmp(&self, other: &ValueWithCost<T>) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl <T: Eq> PartialOrd for ValueWithCost<T> {
    fn partial_cmp(&self, other: &ValueWithCost<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn search<'a, T: Clone + Debug + Hash + Eq, HF: Fn(&T) -> Cost, MF: Fn(&T) -> Vec<T>>(start: &'a T, h_fn: HF, move_fn: MF) -> AStarResult<T> {
    let mut frontier: BinaryHeap<ValueWithCost<T>> = BinaryHeap::new();
    frontier.push(ValueWithCost {
        value: start.to_owned(),
        cost: h_fn(&start)
    });
    let mut previous: HashMap<T, T> = HashMap::new();
    let mut path_cost: HashMap<T, Cost> = HashMap::new();
    path_cost.insert(start.to_owned(), 0);


    while let Some(ValueWithCost { value, cost }) = frontier.pop() {
//        println!("frontier = {:?}", frontier);
        if h_fn(&value) == 0 {
            return AStarResult::from_path(value, previous);
        }

        let new_cost = get_cost(&value, &path_cost).unwrap() + 1;
        for new_value in move_fn(&value).into_iter() {
//            println!("New candidate: {:?}, new_cost={}", new_value, new_cost);
            let current_cost = get_cost(&new_value, &path_cost);

            if current_cost.is_none() || new_cost < current_cost.unwrap() {
//                println!("Better than current: {:?}", current_cost);
                let nv = new_value.to_owned();

                frontier.push(ValueWithCost {
                    value: nv.to_owned(),
                    cost: new_cost + h_fn(&new_value)
                });

                path_cost.insert(nv.to_owned(), new_cost);
                previous.insert(nv.to_owned(), value.to_owned());
            }
        }
    }


    AStarResult {
        failed: true,
        path: vec![]
    }
}

fn get_cost<T: Clone + Hash + Eq>(value: &T, cost_map: &HashMap<T, Cost>) -> Option<u32> {
    match cost_map.get(value) {
        Some(cost) => Some(cost.clone()),
        _ => None
    }
}