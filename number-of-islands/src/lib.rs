use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        if m == 0 {
            return 0;
        }
        let n = grid[0].len();
        if n == 0 {
            return 0;
        }

        let mut visited = HashSet::new();
        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                if Self::dfs(&grid, i, j, &mut visited) {
                    res += 1;
                }
            }
        }
        res
    }

    fn dfs(
        grid: &Vec<Vec<char>>,
        i: usize,
        j: usize,
        visited: &mut HashSet<(usize, usize)>,
    ) -> bool {
        if visited.contains(&(i, j)) {
            return false;
        }
        visited.insert((i, j));
        if grid[i][j] == '1' {
            if i < grid.len() - 1 {
                Self::dfs(&grid, i + 1, j, visited);
            }
            if i > 0 {
                Self::dfs(&grid, i - 1, j, visited);
            }
            if j < grid[0].len() - 1 {
                Self::dfs(&grid, i, j + 1, visited);
            }
            if j > 0 {
                Self::dfs(&grid, i, j - 1, visited);
            }
            return true;
        } else {
            return false;
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
