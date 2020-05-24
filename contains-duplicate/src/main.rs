use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut appeared = HashSet::new();
    for i in &nums {
        if appeared.contains(i) {
            return true;
        } else {
            appeared.insert(i);
        }
    }
    false
}

fn main() {
    println!("Hello, world!");
}
