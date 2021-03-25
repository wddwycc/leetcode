use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return vec![];
        }

        let m = matrix.len() - 1;
        let n = matrix[0].len() - 1;

        let mut res = vec![];
        for x in 0..=m {
            for y in 0..=n {
                let reachable = Self::dfs(&matrix, (m, n), (x, y), &mut HashSet::new());
                if reachable.0 && reachable.1 {
                    res.push(vec![x as i32, y as i32]);
                }
            }
        }
        res
    }

    fn dfs(
        matrix: &[Vec<i32>],
        bounds: (usize, usize),
        pos: (usize, usize),
        visited: &mut HashSet<(usize, usize)>,
    ) -> (bool, bool) {
        visited.insert(pos);

        let (m, n) = bounds;
        let (x, y) = pos;
        let height = matrix[x][y];

        let mut can_go_pacific = false;
        let mut can_go_atlantic = false;

        // NOTE: If on the edge, mark as reachable
        if x == 0 || y == 0 {
            can_go_pacific = true;
        }
        if x == m || y == n {
            can_go_atlantic = true;
        }
        // NOTE: Explore 4 directions
        let mut pos_to_explore = vec![];
        if x > 0 && height >= matrix[x - 1][y] {
            pos_to_explore.push((x - 1, y));
        }
        if x < m && height >= matrix[x + 1][y] {
            pos_to_explore.push((x + 1, y));
        }
        if y > 0 && height >= matrix[x][y - 1] {
            pos_to_explore.push((x, y - 1));
        }
        if y < n && height >= matrix[x][y + 1] {
            pos_to_explore.push((x, y + 1));
        }
        for next_pos in pos_to_explore {
            if can_go_pacific && can_go_atlantic {
                break;
            };
            if visited.contains(&next_pos) {
                continue;
            }
            let reachable = Self::dfs(matrix, bounds, next_pos, visited);
            if reachable.0 {
                can_go_pacific = true
            };
            if reachable.1 {
                can_go_atlantic = true
            };
        }

        (can_go_pacific, can_go_atlantic)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
