use std::collections::{BTreeMap, HashMap};

#[derive(Default)]
struct TimeMap {
    tm: HashMap<String, BTreeMap<i32, String>>,
}

impl TimeMap {
    fn new() -> Self {
        Default::default()
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.tm
            .entry(key)
            .or_insert(BTreeMap::new())
            .insert(timestamp, value);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some((_, v)) = self.tm[&key].range(0..=timestamp).rev().next() {
            v.clone()
        } else {
            "".to_owned()
        }
    }
}
