use std::collections::{HashMap, HashSet};

pub struct Solution {}
impl Solution {
    // time: O(n), space: O(n)
    pub fn two_sum(nums: &[i32], sum: i32) -> Vec<(i32, i32)> {
        let mut res = vec![];
        let mut remainders = HashMap::new();
        for i in 0..nums.len() {
            let val = nums[i];
            if let Some(&prev_i) = remainders.get(&val) {
                res.push((nums[prev_i], val));
            } else {
                remainders.insert(sum - val, i);
            }
        }
        res
    }

    // time: O(n ** 2), space: O(n)
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        if len < 3 {
            return vec![];
        }

        let mut visited = HashSet::new();
        let mut res = vec![];

        nums.sort();
        for idx in 0..(len - 2) {
            let val = nums[idx];
            let remainder = 0 - nums[idx];
            for item in Solution::two_sum(&nums[(idx + 1)..], remainder) {
                let key = (val, item.0, item.1);
                if !visited.contains(&key) {
                    res.push(vec![val, item.0, item.1]);
                    visited.insert(key);
                }
            }
        }
        res
    }
}

fn main() {
    // assert_eq!(Solution::two_sum(&vec![2, 7, 11, 15], 9), vec![vec![0, 1]]);
}
