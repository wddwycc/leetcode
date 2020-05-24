use std::collections::{HashMap, HashSet};

pub struct Solution {}
impl Solution {
    // time: O(n), space: O(n)
    pub fn two_sum(nums: &Vec<i32>, sum: i32, start_idx: usize) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut remainders = HashMap::new();
        for i in start_idx..nums.len() {
            let val = nums[i];
            if let Some(&prev_i) = remainders.get(&val) {
                result.push(vec![nums[prev_i], val]);
            } else {
                remainders.insert(sum - val, i);
            }
        }
        result
    }

    // time: O(n ** 2), space: O(n)
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        if len < 3 {
            return vec![];
        }

        let mut nums = nums;
        let mut result_hashset = HashSet::new();
        let mut result = vec![];

        nums.sort();
        for idx in 0..(len - 2) {
            let val = nums[idx];
            let remainder = 0 - nums[idx];
            let res = Solution::two_sum(&nums, remainder, idx + 1);
            for item in res {
                let item_str = format!("{},{},{}", val, item[0], item[1]);
                if result_hashset.contains(&item_str) {
                    continue;
                } else {
                    result.push(vec![val, item[0], item[1]]);
                    result_hashset.insert(item_str);
                }
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::two_sum(&vec![2, 7, 11, 15], 9, 1000),
        vec![vec![0, 1]]
    );
}
