pub struct Solution;
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        if m == 0 {
            return 0;
        }
        let n = grid[0].len();
        if n == 0 {
            return 0;
        }

        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    Self::dfs(&mut grid, i, j);
                    res += 1;
                }
            }
        }
        res
    }

    fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if grid[i][j] == '0' {
            return;
        }

        grid[i][j] = '0';
        if i < grid.len() - 1 {
            Self::dfs(grid, i + 1, j);
        }
        if i > 0 {
            Self::dfs(grid, i - 1, j);
        }
        if j < grid[0].len() - 1 {
            Self::dfs(grid, i, j + 1);
        }
        if j > 0 {
            Self::dfs(grid, i, j - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
