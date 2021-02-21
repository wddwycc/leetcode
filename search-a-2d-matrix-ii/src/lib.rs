use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // just search with DFS, time should be O(mn), space should be O(m + n)
        let m = matrix.len() - 1;
        let n = matrix[0].len() - 1;
        Self::dfs(&matrix, (m, n), (0, 0), target, &mut HashSet::new())
    }

    fn dfs(
        matrix: &[Vec<i32>],
        bounds: (usize, usize),
        cur: (usize, usize),
        target: i32,
        visited: &mut HashSet<(usize, usize)>,
    ) -> bool {
        if visited.contains(&cur) {
            return false;
        }
        visited.insert(cur);

        let (m, n) = bounds;
        let (x, y) = cur;
        let val = matrix[x][y];
        if val == target {
            return true;
        }
        if x < m && Self::dfs(matrix, bounds, (x + 1, y), target, visited) {
            return true;
        }
        if y < n && Self::dfs(matrix, bounds, (x, y + 1), target, visited) {
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
