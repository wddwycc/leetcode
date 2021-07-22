pub struct Solution;
impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        // dp
        // f(i, j) = min { f(i - 1, j + 1), f(i, j + 1), f(i + 1, j + 1) } + matrix[i][j];

        let m = matrix.len();
        let n = matrix[0].len();
        let mut cache = vec![vec![None; n]; m];
        (0..n)
            .map(|j| Self::dfs(&matrix, (m, n), (0, j), &mut cache))
            .min()
            .unwrap()
    }

    fn dfs(
        matrix: &[Vec<i32>],
        bounds: (usize, usize),
        pos: (usize, usize),
        cache: &mut Vec<Vec<Option<i32>>>,
    ) -> i32 {
        let (m, n) = bounds;
        let (i, j) = pos;
        if let Some(cached) = cache[i][j] {
            return cached;
        }

        let mut ans = matrix[i][j];
        if i + 1 < m {
            let mut acc = Self::dfs(matrix, bounds, (i + 1, j), cache);
            if j > 0 {
                acc = acc.min(Self::dfs(matrix, bounds, (i + 1, j - 1), cache));
            }
            if j + 1 < n {
                acc = acc.min(Self::dfs(matrix, bounds, (i + 1, j + 1), cache));
            }
            ans += acc;
        }
        cache[i][j] = Some(ans);
        ans
    }
}
