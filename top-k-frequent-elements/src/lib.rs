use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }
        let mut dist: Vec<(i32, i32)> = map
            .into_iter()
            .map(|(a, b)| (a, b))
            .collect::<Vec<(i32, i32)>>();
        dist.sort_by(|a, b| a.1.cmp(&b.1).reverse());
        let mut res = vec![];
        for idx in 0..k {
            res.push(dist[idx as usize].0);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        );
    }
}
