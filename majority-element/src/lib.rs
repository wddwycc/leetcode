use std::collections::HashMap;
pub struct Solution {}
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut dist = HashMap::new();
        for num in nums {
            let entry = dist.entry(num).or_insert(0);
            *entry += 1;
        }
        let mut res = (0, 0);
        for (k, v) in dist.iter() {
            if v > &res.1 {
                res = (*k, *v)
            }
        }
        res.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
