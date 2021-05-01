use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut dist = HashMap::new();
        for num in nums {
            let offset = num - Self::rev(num);
            *dist.entry(offset).or_insert(0) += 1;
        }
        let mut res: i64 = dist
            .into_iter()
            .filter(|&(_, ct)| ct > 1)
            .map(|(_, ct)| ct as i64)
            .map(|ct| ct * (ct - 1) / 2)
            .sum();
        res %= 1_000_000_007;
        res as i32
    }

    fn rev(mut num: i32) -> i32 {
        let mut res = 0;
        while num > 0 {
            res *= 10;
            res += num % 10;
            num /= 10;
        }
        res
    }
}
