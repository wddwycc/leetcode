pub struct Solution;
impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len() - 1;
        let n = grid[0].len() - 1;

        // give function f(i, j) represents: for grid[i][j], min sum to reach right-botom
        // if i < m && j < n, f(i, j) = grid[i][j] + min { f(i + 1, j), f(i, j + 1) }
        // if i < m         , f(i, j) = grid[i][j] + f(i + 1, j)
        // if          j < n, f(i, j) = grid[i][j] + f(i, j + 1)
        // else             , f(i, j) = grid[i][j]  // bottom-case

        for i in (0..=m).rev() {
            for j in (0..=n).rev() {
                grid[i][j] = grid[i][j] + {
                    if i == m && j == n {
                        0
                    } else if i == m {
                        grid[i][j + 1]
                    } else if j == n {
                        grid[i + 1][j]
                    } else {
                        std::cmp::min(grid[i + 1][j], grid[i][j + 1])
                    }
                }
            }
        }
        grid[0][0]
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
