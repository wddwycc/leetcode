pub struct Solution;
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len() - 1;
        let n = grid[0].len() - 1;

        let mut res = 0;
        let mut visited = vec![vec![false; n + 1]; m + 1];
        for i in 0..=m {
            for j in 0..=n {
                if grid[i][j] == 0 {
                    continue;
                }
                res = res.max(Self::dfs(&grid, (m, n), (i, j), &mut visited));
            }
        }
        res
    }

    fn dfs(
        grid: &[Vec<i32>],
        bounds: (usize, usize),
        cur: (usize, usize),
        visited: &mut [Vec<bool>],
    ) -> i32 {
        let (m, n) = bounds;
        let (x, y) = cur;
        if visited[x][y] {
            return 0;
        }
        visited[x][y] = true;

        let mut res = 1;
        if x > 0 && grid[x - 1][y] == 1 {
            res += Self::dfs(grid, bounds, (x - 1, y), visited);
        }
        if x < m && grid[x + 1][y] == 1 {
            res += Self::dfs(grid, bounds, (x + 1, y), visited);
        }
        if y > 0 && grid[x][y - 1] == 1 {
            res += Self::dfs(grid, bounds, (x, y - 1), visited);
        }
        if y < n && grid[x][y + 1] == 1 {
            res += Self::dfs(grid, bounds, (x, y + 1), visited);
        }
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
