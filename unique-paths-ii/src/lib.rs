pub struct Solution;
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();

        if obstacle_grid[0][0] == 0 || obstacle_grid[m - 1][n - 1] == 0 {
            return 0;
        }

        let mut res = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                match (i, j) {
                    (0, 0) => res[i][j] = 1,
                    (0, j) => {
                        if obstacle_grid[i][j - 1] == 0 {
                            res[i][j] = res[i][j - 1];
                        }
                    }
                    (i, 0) => {
                        if obstacle_grid[i - 1][j] == 0 {
                            res[i][j] = res[i - 1][j]
                        }
                    }
                    (i, j) => {
                        let mut val = 0;
                        if obstacle_grid[i - 1][j] == 0 {
                            val += res[i - 1][j];
                        }
                        if obstacle_grid[i][j - 1] == 0 {
                            val += res[i][j - 1]
                        }
                        res[i][j] = val;
                    }
                }
            }
        }
        res[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
