use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        // give function: f(days[i]) = v, represents min cost from days[i] to the end.
        // f(i) = min {
        //    f(i + 1) + costs[0],  if i == n, just return costs[0]
        //    f(k) + costs[1], k is the smallest number satisfies days[k] >= days[i] + 7, if k not exist, just return costs[1]
        //    f(k) + costs[2], k is the smallest number satisfies days[k] >= days[i] + 30, if k not exist, just return costs[2]
        // }

        // let's try top-down dp, do memorization for f(i)
        let mut cache = HashMap::new();
        Self::dfs(&days, &costs, 0, &mut cache)
    }

    fn dfs(days: &[i32], costs: &[i32], i: usize, cache: &mut HashMap<usize, i32>) -> i32 {
        if let Some(cached) = cache.get(&i) {
            return *cached;
        }
        let cost1 = {
            let mut res = costs[0];
            if i + 1 < days.len() {
                res += Self::dfs(days, costs, i + 1, cache);
            }
            res
        };
        let cost2 = {
            let mut res = costs[1];
            res += days[(i + 1)..]
                .iter()
                .enumerate()
                .find(|(_, a)| **a >= days[i] + 7)
                .map(|(idx, _)| Self::dfs(days, costs, i + idx + 1, cache))
                .unwrap_or(0);
            res
        };
        let cost3 = {
            let mut res = costs[2];
            res += days[(i + 1)..]
                .iter()
                .enumerate()
                .find(|(_, a)| **a >= days[i] + 30)
                .map(|(idx, _)| Self::dfs(days, costs, i + idx + 1, cache))
                .unwrap_or(0);
            res
        };
        let res = cost1.min(cost2).min(cost3);
        cache.insert(i, res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]),
            17
        );
    }
}
