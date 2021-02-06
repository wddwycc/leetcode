use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // give f(s) = k, represents to sum up to s, min coin nums is k

        // let's try top-down DP
        // memoization: HashMap<s, k>
        let mut cache = HashMap::new();
        Self::dfs(&coins, amount, 0, &mut cache)
            .map(|a| a as i32)
            .unwrap_or(-1)
    }

    fn dfs(
        coins: &[i32],
        target: i32,
        acc: usize,
        cache: &mut HashMap<i32, Result<usize, ()>>,
    ) -> Result<usize, ()> {
        if let Some(cached) = cache.get(&target) {
            return *cached;
        }

        if target == 0 {
            return Ok(acc);
        }
        if target < 0 {
            return Err(());
        }
        let res = coins
            .iter()
            .filter_map(|c| Self::dfs(coins, target - c, 1, cache).ok())
            .min()
            .map(|a| a + acc)
            .ok_or(());
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
