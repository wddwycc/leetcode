use std::collections::HashSet;

pub struct Solution;
impl Solution {
    fn dfs(
        src: &[i32],
        cur: usize,
        mut consumed: Vec<i32>,
        target: i32,
        res: &mut HashSet<Vec<i32>>,
    ) {
        if target == 0 {
            consumed.sort();
            res.insert(consumed);
            return;
        }
        if cur >= src.len() {
            return;
        }
        // option1: consume cur, go next
        {
            let val = src[cur];
            if val <= target {
                let mut next_consumed = consumed.clone();
                next_consumed.push(val);
                Self::dfs(src, cur + 1, next_consumed, target - val, res);
            }
        }
        // option2: skip cur, go next
        Self::dfs(src, cur + 1, consumed, target, res);
    }

    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = HashSet::new();
        Self::dfs(&candidates, 0, vec![], target, &mut res);
        res.into_iter().collect()
    }
}
