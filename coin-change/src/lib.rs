use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // give f(s) = k, represents to sum up to s, min coin nums is k

        // let's try top-down DP
        // memoization: HashMap<s, k>
        let mut cache = HashMap::new();
        Self::dfs(&coins, amount, &mut cache)
            .map(|a| a as i32)
            .unwrap_or(-1)
    }

    fn dfs(coins: &[i32], target: i32, cache: &mut HashMap<i32, Option<usize>>) -> Option<usize> {
        if let Some(cached) = cache.get(&target) {
            return *cached;
        }

        if target == 0 {
            return Some(0);
        }
        if target < 0 {
            return None;
        }
        let res = coins
            .iter()
            .filter_map(|c| Self::dfs(coins, target - c, cache))
            .min()
            .map(|a| a + 1);
        cache.insert(target, res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }
}
