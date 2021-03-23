use std::collections::{HashMap, HashSet};

pub struct Solution;
impl Solution {
    // time: O(n), space: O(n)
    pub fn two_sum(nums: &[i32], sum: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut remainders = HashMap::new();
        for i in 0..nums.len() {
            let val = nums[i];
            if let Some(&prev_i) = remainders.get(&val) {
                res.push(vec![nums[prev_i], val]);
            } else {
                remainders.insert(sum - val, i);
            }
        }
        res
    }

    // time: O(n ** 2), space: O(n)
    pub fn three_sum_multi(mut arr: Vec<i32>, target: i32) -> i32 {
        let len = arr.len();
        if len < 3 {
            return 0;
        }

        let arr_dist = {
            let mut res = HashMap::new();
            for i in 0..len {
                *res.entry(arr[i]).or_insert(0) += 1;
            }
            res
        };

        let mut visited = HashSet::new();
        let mut res = 0;

        arr.sort();
        for idx in 0..(arr.len() - 2) {
            let remainder = target - arr[idx];
            for two_sum_match in Solution::two_sum(&arr[(idx + 1)..], remainder) {
                let item_0 = arr[idx];
                let item_1 = two_sum_match[0];
                let item_2 = two_sum_match[1];
                if visited.contains(&(item_0, item_1, item_2)) {
                    continue;
                } else {
                    let dist = {
                        let mut res = HashMap::new();
                        for item in &[item_0, item_1, item_2] {
                            *res.entry(*item).or_insert(0) += 1;
                        }
                        res
                    };

                    let count: i64 = dist
                        .iter()
                        .map(|(num, num_count)| {
                            let mut n = *arr_dist.get(num).unwrap() as i64;
                            match num_count {
                                1 => n,
                                2 => n * (n - 1) / 2,
                                3 => {
                                    let mut res = 0;
                                    while n > 2 {
                                        res += (n - 1) * (n - 2) / 2;
                                        n -= 1;
                                    }
                                    res
                                }
                                _ => panic!(),
                            }
                        })
                        .product();
                    res += count;
                    visited.insert((item_0, item_1, item_2));
                }
            }
        }

        let modulo = 10_i64.pow(9) + 7;
        (res % modulo) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::three_sum_multi(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 8),
            20
        );
    }
}
