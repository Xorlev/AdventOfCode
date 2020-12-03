use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct FrequencyMap<T: Eq + Hash> {
    counts: HashMap<T, u64>
}

impl<T: Eq + Hash> FrequencyMap<T> {
    pub fn new() -> Self {
        FrequencyMap {
            counts: HashMap::new()
        }
    }

    pub fn add(&mut self, value: T) {
        match self.counts.entry(value) {
            Entry::Vacant(count) => { count.insert(1); },
            Entry::Occupied(mut count) => { *count.get_mut() += 1; },
        }
    }

    pub fn count(&self, value: &T) -> u64 {
        self.counts.get(value).cloned().unwrap_or(0)
    }
}

impl<T: Eq + Hash> FromIterator<T> for FrequencyMap<T> {
    fn from_iter<I: IntoIterator<Item=T>>(it: I) -> FrequencyMap<T> {
        let mut v = FrequencyMap::new();
        v.extend(it);
        v
    }
}

impl<T: Eq + Hash> Extend<T> for FrequencyMap<T> {
    fn extend<I: IntoIterator<Item=T>>(&mut self, it: I) {
        for sample in it {
            self.add(sample);
        }
    }
}