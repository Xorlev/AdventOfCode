extern crate regex;
extern crate stats;

use std::cmp::Ordering;

fn main() {
    use std::io;
    use regex::Regex;
    use std::io::prelude::*;
    use std::collections::HashSet;
    use std::collections::hash_map::{HashMap, Entry};

    let stdin = io::stdin();

    //ejpanjwpekjwh-xwogap-odellejc-654[ejpwa]
    let re = Regex::new(r"([a-z-]+)-(\d+)\[([a-z]+)\]").unwrap();
    let mut sum = 0i32;
    let mut freq: HashMap<char, u32> = HashMap::with_capacity(26);
    for line in stdin.lock().lines() {
        let input = line.unwrap();
        let caps = re.captures(input.as_str()).unwrap();

        for ch in caps[1].chars() {
            if ch != '-' {
                match freq.entry(ch) {
                    Entry::Vacant(count) => { count.insert(1); },
                    Entry::Occupied(mut count) => { *count.get_mut() += 1; },
                }
            }
        }

        let mut counts: Vec<(char, u32)> = freq.iter().map(|(&k, &v)| (k, v)).collect();
        counts.sort_by(|l, r| order(l,r));

        let most_frequent_set: HashSet<char> = counts.into_iter()
            .take(5)
            .map(|c| c.0)
            .collect();

        if caps[3].chars().all(|c| most_frequent_set.contains(&c)) {
            sum += caps[2].parse::<i32>().unwrap();
        }

        freq.clear()
    }

    println!("{:?}", sum);
}

fn order(l: &(char, u32), r: &(char, u32)) -> Ordering {
    let freq_order = r.1.cmp(&l.1);

    if freq_order == Ordering::Equal {
        l.0.cmp(&r.0)
    } else {
        freq_order
    }
}
