use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return vec![];
        }

        let m = matrix.len() - 1;
        let n = matrix[0].len() - 1;

        let pacific_pos = {
            let mut res = HashSet::new();
            res.insert((0, 0));
            for x in 1..=m {
                Self::dfs(&matrix, (m, n), (x, 0), &mut res);
            }
            for y in 1..=n {
                Self::dfs(&matrix, (m, n), (0, y), &mut res);
            }
            res
        };
        let atlantic_pos = {
            let mut res = HashSet::new();
            res.insert((m, n));
            for x in 1..=m {
                Self::dfs(&matrix, (m, n), (x, n), &mut res);
            }
            for y in 1..=n {
                Self::dfs(&matrix, (m, n), (m, y), &mut res);
            }
            res
        };

        pacific_pos
            .intersection(&atlantic_pos)
            .map(|(x, y)| vec![*x as i32, *y as i32])
            .collect()
    }

    fn dfs(
        matrix: &[Vec<i32>],
        bounds: (usize, usize),
        pos: (usize, usize),
        visited: &mut HashSet<(usize, usize)>,
    ) {
        if visited.contains(&pos) {
            return;
        }
        visited.insert(pos);

        let (m, n) = bounds;
        let (x, y) = pos;
        let height = matrix[x][y];

        // NOTE: Explore 4 directions
        let mut pos_to_explore = vec![];
        if x > 0 && height <= matrix[x - 1][y] {
            pos_to_explore.push((x - 1, y));
        }
        if x < m && height <= matrix[x + 1][y] {
            pos_to_explore.push((x + 1, y));
        }
        if y > 0 && height <= matrix[x][y - 1] {
            pos_to_explore.push((x, y - 1));
        }
        if y < n && height <= matrix[x][y + 1] {
            pos_to_explore.push((x, y + 1));
        }
        for next_pos in pos_to_explore {
            if visited.contains(&next_pos) {
                continue;
            }
            Self::dfs(matrix, bounds, next_pos, visited);
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
