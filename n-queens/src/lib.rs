pub struct Solution;
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut res = vec![];
        let mut queens = vec![];
        Self::dfs(n, &mut queens, &mut res, (0, 0));
        res
    }

    fn dfs(n: i32, queens: &mut Vec<(i32, i32)>, res: &mut Vec<Vec<String>>, cur: (i32, i32)) {
        if cur.0 == n {
            if queens.len() as i32 == n {
                let mut grid = vec![vec!['.'; n as usize]; n as usize];
                for queen in queens {
                    grid[queen.0 as usize][queen.1 as usize] = 'Q';
                }
                let grid = grid
                    .into_iter()
                    .map(|a| a.iter().collect::<String>())
                    .collect::<Vec<_>>();
                res.push(grid);
            }
            return;
        }
        // NOTE: try place on cur
        if Self::is_not_under_attack(&queens, cur) {
            queens.push(cur);
            Self::dfs(n, queens, res, (cur.0 + 1, 0));
            queens.pop();
        }
        // NOTE: try skip cur
        if cur.1 < n - 1 {
            Self::dfs(n, queens, res, (cur.0, cur.1 + 1));
        } else {
            Self::dfs(n, queens, res, (cur.0 + 1, 0));
        }
    }

    fn is_not_under_attack(queens: &[(i32, i32)], pos: (i32, i32)) -> bool {
        for queen in queens {
            if pos.0 == queen.0
                || pos.1 == queen.1
                || ((pos.0 - queen.0).abs() == (pos.1 - queen.1).abs())
            {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
