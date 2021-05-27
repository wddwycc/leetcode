use std::cmp::Reverse;
use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut dist = HashMap::new();
        for c in s.chars() {
            *dist.entry(c).or_insert(0) += 1;
        }
        let mut dist: Vec<(char, usize)> = dist.into_iter().collect();
        dist.sort_by_key(|a| Reverse(a.1));
        let mut res = "".to_owned();
        for (c, n) in dist {
            for _ in 0..n {
                res.push(c)
            }
        }
        res
    }
}
