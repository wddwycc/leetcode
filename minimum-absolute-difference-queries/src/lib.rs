use std::collections::BTreeSet;

pub struct Solution;
impl Solution {
    pub fn min_difference(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut results = vec![vec![0; n]; n];
        for i in 0..n {
            let mut min_diff = std::i32::MAX;
            let mut tree = BTreeSet::new();
            for j in i..n {
                tree.insert(nums[j]);
                if tree.iter().next() == tree.iter().next_back() {
                    results[i][j] = -1;
                    continue;
                }
                // search left
                for &l_v in tree.range(..nums[j]).rev() {
                    if nums[j] == l_v {
                        continue;
                    }
                    min_diff = min_diff.min(nums[j] - l_v);
                    break;
                }
                // search right
                for &r_v in tree.range((nums[j] + 1)..) {
                    if nums[j] == r_v {
                        continue;
                    }
                    min_diff = min_diff.min(r_v - nums[j]);
                    break;
                }
                results[i][j] = min_diff;
            }
        }

        queries
            .into_iter()
            .map(|q| {
                let l = q[0] as usize;
                let r = q[1] as usize;
                results[l][r]
            })
            .collect()
    }
}
