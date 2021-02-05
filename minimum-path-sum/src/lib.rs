use std::collections::HashMap;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len() - 1;
        let n = grid[0].len() - 1;

        // brute-force search, time complexity would be O(2 ** (m+n)), time exceed

        // give function f(i, j) represents: for grid[i][j], min sum to reach right-botom
        // if i < m && j < n, f(i, j) = grid[i][j] + min { f(i + 1, j), f(i, j + 1) }
        // if i < m         , f(i, j) = grid[i][j] + f(i + 1, j)
        // if          j < n, f(i, j) = grid[i][j] + f(i, j + 1)
        // else             , f(i, j) = grid[i][j]  // bottom-case

        // let's try top-down dp
        // HashMap<(i, j), i32>, memorizes result of f(i, j)

        let mut cache = HashMap::new();
        Self::dfs(&grid, (m, n), (0, 0), &mut cache)
    }

    fn dfs(
        grid: &[Vec<i32>],
        bounds: (usize, usize),
        cur: (usize, usize),
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if let Some(res) = cache.get(&cur) {
            return *res;
        }

        let (i, j) = cur;
        let (m, n) = bounds;
        let mut res = {
            if i < m && j < n {
                std::cmp::min(
                    Self::dfs(grid, bounds, (i + 1, j), cache),
                    Self::dfs(grid, bounds, (i, j + 1), cache),
                )
            } else if i < m {
                Self::dfs(grid, bounds, (i + 1, j), cache)
            } else if j < n {
                Self::dfs(grid, bounds, (i, j + 1), cache)
            } else {
                0
            }
        };
        res += grid[i][j];
        cache.insert(cur, res);
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
