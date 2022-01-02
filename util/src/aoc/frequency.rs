use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct FrequencyMap<T: Eq + Hash> {
    counts: HashMap<T, u64>,
}

impl<T: Eq + Hash> FrequencyMap<T> {
    pub fn new() -> Self {
        FrequencyMap {
            counts: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: T) {
        self.multi_add(key, 1)
    }

    pub fn multi_add(&mut self, key: T, value: u64) {
        match self.counts.entry(key) {
            Entry::Vacant(count) => {
                count.insert(value);
            }
            Entry::Occupied(mut count) => {
                *count.get_mut() += value;
            }
        }
    }

    pub fn count(&self, value: &T) -> u64 {
        self.counts.get(value).cloned().unwrap_or(0)
    }

    pub fn keys(&self) -> impl Iterator<Item = &T> {
        self.counts.keys()
    }

    pub fn entries(&self) -> impl Iterator<Item = (&T, &u64)> {
        self.counts.iter()
    }

    pub fn min(&self) -> Option<&T> {
        self.counts
            .iter()
            .min_by_key(|entry| entry.1)
            .map(|(k, _v)| k)
    }

    pub fn max(&self) -> Option<&T> {
        self.counts
            .iter()
            .max_by_key(|entry| entry.1)
            .map(|(k, _v)| k)
    }
}

impl<T: Eq + Hash> FromIterator<T> for FrequencyMap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(it: I) -> FrequencyMap<T> {
        let mut v = FrequencyMap::new();
        v.extend(it);
        v
    }
}

impl<T: Eq + Hash> Extend<T> for FrequencyMap<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, it: I) {
        for sample in it {
            self.add(sample);
        }
    }
}
