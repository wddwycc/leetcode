use std::collections::LinkedList;

const BUCKET_COUNT: usize = 100;

#[derive(Default)]
struct MyHashMap {
    buckets: Vec<LinkedList<(i32, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            buckets: vec![LinkedList::new(); BUCKET_COUNT],
        }
    }

    fn key_hash(key: i32) -> usize {
        (key as usize) % BUCKET_COUNT
    }

    /** value will always be non-negative. */
    fn put(&mut self, key: i32, value: i32) {
        let hash = Self::key_hash(key);
        for node in self.buckets[hash].iter_mut() {
            if node.0 == key {
                node.1 = value;
                return;
            }
        }
        self.buckets[hash].push_back((key, value));
    }

    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    fn get(&self, key: i32) -> i32 {
        let hash = Self::key_hash(key);
        for node in self.buckets[hash].iter() {
            if node.0 == key {
                return node.1;
            }
        }
        -1
    }

    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    fn remove(&mut self, key: i32) {
        let hash = Self::key_hash(key);
        let idx = self.buckets[hash]
            .iter()
            .enumerate()
            .find_map(|(idx, node)| if node.0 == key { Some(idx) } else { None });
        if let Some(idx) = idx {
            let mut split_list = self.buckets[hash].split_off(idx);
            split_list.pop_front();
            self.buckets[hash].append(&mut split_list);
        }
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
