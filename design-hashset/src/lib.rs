pub struct MyHashSet {
    buckets: Vec<Vec<i32>>,
}

impl MyHashSet {
    pub fn new() -> Self {
        Self {
            buckets: vec![vec![]; 10_000],
        }
    }

    pub fn add(&mut self, key: i32) {
        let hash = (key % 100) as usize;
        let bucket = self.buckets.get_mut(hash).unwrap();
        let exists = bucket.iter().any(|a| a == &key);
        if !exists {
            bucket.push(key);
        }
    }

    pub fn remove(&mut self, key: i32) {
        let hash = (key % 100) as usize;
        let bucket = self.buckets.get_mut(hash).unwrap();
        let idx = match bucket.iter().enumerate().find(|(_, v)| v == &&key) {
            Some(a) => a.0,
            None => return,
        };
        bucket.remove(idx);
    }

    pub fn contains(&self, key: i32) -> bool {
        let hash = (key % 100) as usize;
        let bucket = self.buckets.get(hash).unwrap();
        bucket.iter().any(|a| a == &key)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
