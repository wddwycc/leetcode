pub struct Solution;
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let chars: Vec<char> = word.chars().collect();
        let m = board.len() - 1;
        let n = board[0].len() - 1;

        let mut visited = vec![vec![false; n + 1]; m + 1];
        for i in 0..=m {
            for j in 0..=n {
                if Self::dfs(&chars, &board, (m, n), (i, j), &mut visited) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(
        chars: &[char],
        board: &[Vec<char>],
        bounds: (usize, usize),
        cur: (usize, usize),
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        let (m, n) = bounds;
        let (i, j) = cur;

        if visited[i][j] {
            return false;
        }
        if chars[0] != board[i][j] {
            return false;
        }
        if chars.len() == 1 {
            return true;
        }
        visited[i][j] = true;
        if i > 0 {
            if Self::dfs(&chars[1..], board, bounds, (i - 1, j), visited) {
                return true;
            }
        }
        if i < m {
            if Self::dfs(&chars[1..], board, bounds, (i + 1, j), visited) {
                return true;
            }
        }
        if j > 0 {
            if Self::dfs(&chars[1..], board, bounds, (i, j - 1), visited) {
                return true;
            }
        }
        if j < n {
            if Self::dfs(&chars[1..], board, bounds, (i, j + 1), visited) {
                return true;
            }
        }
        visited[i][j] = false;
        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
