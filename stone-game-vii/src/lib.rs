pub struct Solution;
impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let prefix_sum = {
            let mut res = vec![0; n + 1];
            for i in 1..=n {
                res[i] = res[i - 1] + stones[i - 1];
            }
            res
        };
        // dp[i, j] => for player facing range [i, j], best score advantage he can achieve.
        // dp[i, j] = max { sum { i + 1, j } - dp[i + 1, j], sum { i, j - 1 } - dp[i, j - 1] };
        // when i == j, dp[i, j] = 0;
        let mut cache = vec![vec![None; n]; n];
        Self::calc(&stones, &prefix_sum, (0, n - 1), &mut cache)
    }

    fn calc(
        stones: &[i32],
        prefix_sum: &[i32],
        cur: (usize, usize),
        cache: &mut Vec<Vec<Option<i32>>>,
    ) -> i32 {
        let (i, j) = cur;
        if let Some(cached) = cache[i][j] {
            return cached;
        }
        if i == j {
            return 0;
        }
        let res = std::cmp::max(
            prefix_sum[j + 1]
                - prefix_sum[i + 1]
                - Self::calc(stones, prefix_sum, (i + 1, j), cache),
            prefix_sum[j] - prefix_sum[i] - Self::calc(stones, prefix_sum, (i, j - 1), cache),
        );
        cache[i][j] = Some(res);
        res
    }
}
