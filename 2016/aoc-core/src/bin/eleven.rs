extern crate all;

use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

type Floor = u32;

trait ItemClass {
    fn generator(self) -> Option<Item>;
    fn item(self) -> Option<Item>;
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Item {
    TG, TM,
    PLG, PLM,
    PRG, PRM,
    SG, SM,
    RG, RM,
    EG, EM,
    DG, DM
}

impl ItemClass for Item {
    fn generator(self) -> Option<Item> {
        match self {
            Item::TM => Some(Item::TG),
            Item::PLM => Some(Item::PLG),
            Item::PRM => Some(Item::PRG),
            Item::PRM => Some(Item::PRG),
            Item::SM => Some(Item::SG),
            Item::RM => Some(Item::RG),
            Item::DM => Some(Item::DG),
            _ => None
        }
    }

    fn item(self) -> Option<Item> {
        match self {
            Item::TG => Some(Item::TM),
            Item::PLG => Some(Item::PLM),
            Item::PRG => Some(Item::PRM),
            Item::PRG => Some(Item::PRM),
            Item::SG => Some(Item::SM),
            Item::RG => Some(Item::RM),
            Item::DG => Some(Item::DM),
            _ => None
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct State {
    elevator: Floor,
    floors: Vec<HashSet<Item>>
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.elevator);
        let mut statehash = 0u64;
        for floor in &self.floors {
            for item in floor {
                let mut hasher = DefaultHasher::new();
                item.hash(&mut hasher);
                statehash ^= hasher.finish();
            }
        }
        state.write_u64(statehash);
    }
}

impl State {
    fn items_on_floor(&self) -> &HashSet<Item> {
        return &self.floors.get(self.elevator as usize).unwrap();
    }

    fn move_items_to_floor(&self, new_floor: Floor, items: &HashSet<Item>) -> Option<State> {
        let mut new_floors: Vec<HashSet<Item>> = vec![];
        for i in 0..self.floors.len() {
            let floor_items: &HashSet<Item> = self.floors.get(i).unwrap();

            let new_items: HashSet<Item> = if i as u32 == self.elevator {
                floor_items.difference(items).map(Item::clone).collect()
            } else if i as u32 == new_floor {
                floor_items.union(items).map(Item::clone).collect()
            } else {
                floor_items.clone()
            };
//            println!("{} ({}) floor_items: {:?}", self.elevator, i, floor_items);
//            println!("{} ({}) items: {:?}", self.elevator, i, items);
//            println!("{} ({}) new_items: {:?}", self.elevator, i, new_items);

            if !legal_state(&new_items) {
                return None
            }

            new_floors.push(new_items)
        }

        Some(
            State {
                elevator: new_floor,
                floors: new_floors
            }
        )
    }

    fn cost_to_top(&self) -> u32 {
        self.floors.iter().rev().enumerate()
            .fold(0, |acc, (idx, floor)| acc + (idx*floor.len()*1010) as u32) / 2
    }

    fn move_from(&self) -> Vec<State> {
        let mut states = Vec::new();
        if self.elevator > 0 {
            self.new_states_at_floor(&mut states, self.elevator - 1)
        }
        if self.elevator < 3 {
            self.new_states_at_floor(&mut states, self.elevator + 1)
        }

        states
    }

    fn new_states_at_floor(&self, new_states: &mut Vec<State>, floor: Floor) {
        for item_combo in permutations(self.items_on_floor()) {
            if let Some(new_state) = self.move_items_to_floor(floor, &item_combo) {
                new_states.push(new_state);
            }
        }
    }

}

fn permutations(items: &HashSet<Item>) -> Vec<HashSet<Item>> {
    let mut new_items = Vec::new();

    let mut combos_used = HashSet::new();
    for i1 in items {
        let mut s1 = HashSet::new();
        s1.insert(i1.clone());
        new_items.push(s1);
        for i2 in items {
            let new_combo = (i1, i2);
            if i1 != i2 && !combos_used.contains(&new_combo) {
                let mut s2 = HashSet::new();
                s2.insert(i1.clone());
                s2.insert(i2.clone());
                new_items.push(s2);
                combos_used.insert(new_combo);
                combos_used.insert((i2, i1));
            }
        }
    }

    new_items
}

fn legal_state(floor: &HashSet<Item>) -> bool {
    let generators: HashSet<&Item> = floor.iter()
        .filter(|item| item.generator().is_none())
        .collect();

    if !generators.is_empty() {
        return floor
            .iter()
            .filter(|item| !item.generator().is_none())
            .all(|item| generators.contains(&item.generator().unwrap()));
    }

    true
}

/**
public static double costToTop(State state) {
        double cost = 0;

        List<ImmutableSet<Item>> floors = Lists.reverse(state.floors);

        // Cost should be 0 at the top and multiplied by floor at the bottom
        for(int i = 0; i < floors.size(); i++) {
            cost += floors.get(i).size()*i;
        }

        return cost / 2; // up to two items per move
    }
    */


fn main() {
    use all::aoc::astar;
    /*
            State part1 = new State(0, ImmutableList.of(
                ImmutableSet.of(Item.TG, Item.TM, Item.PLG, Item.SG),
                ImmutableSet.of(Item.PLM, Item.SM),
                ImmutableSet.of(Item.PRG, Item.PRM, Item.RG, Item.RM),
                ImmutableSet.of()
            ));
    */
//    let f1: HashSet<Item> = vec![Item::TG, Item::TM, Item::PLG, Item::SG].into_iter().collect();
//    let f2: HashSet<Item> = vec![Item::PLM, Item::SM].into_iter().collect();
//    let f3: HashSet<Item> = vec![Item::PRG, Item::PRM, Item::RG, Item::RM].into_iter().collect();
//    let f4: HashSet<Item> = vec![].into_iter().collect();
//    let part1 = State {
//        elevator: 0,
//        floors: vec![f1, f2, f3, f4]
//    };
    let f1: HashSet<Item> = vec![Item::TG, Item::TM, Item::PLG, Item::SG, Item::EG, Item::EM, Item::DG, Item::DM].into_iter().collect();
    let f2: HashSet<Item> = vec![Item::PLM, Item::SM].into_iter().collect();
    let f3: HashSet<Item> = vec![Item::PRG, Item::PRM, Item::RG, Item::RM].into_iter().collect();
    let f4: HashSet<Item> = vec![].into_iter().collect();
    let part1 = State {
        elevator: 0,
        floors: vec![f1, f2, f3, f4]
    };
//    let f1: HashSet<Item> = vec![Item::RG, Item::TG, Item::TM].into_iter().collect();
//    let f2: HashSet<Item> = vec![].into_iter().collect();
//    let f3: HashSet<Item> = vec![Item::RM].into_iter().collect();
//    let f4: HashSet<Item> = vec![].into_iter().collect();
//    let part1 = State {
//        elevator: 0,
//        floors: vec![f1, f2, f3, f4]
//    };


    let result: astar::AStarResult<State> = astar::search(&part1, State::cost_to_top, State::move_from);
    println!("Test: {:?}", result);
    println!("Test: {:?}", result.path.len() - 1);
}