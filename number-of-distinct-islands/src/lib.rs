use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn num_distinct_islands(grid: Vec<Vec<i32>>) -> i32 {
        // Step1: Use dfs to find all grids
        // Step2: Normalize all islands.
        // Step3: Unique sets.
        let islands: Vec<Vec<(usize, usize)>> = Self::find_all_islands(&grid)
            .into_iter()
            .map(|mut island| {
                let min_i = island.iter().map(|(i, _)| i).min().unwrap().to_owned();
                let min_j = island.iter().map(|(_, j)| j).min().unwrap().to_owned();
                for (i, j) in island.iter_mut() {
                    *i -= min_i;
                    *j -= min_j;
                }
                island
            })
            .collect();
        let mut hashset = HashSet::new();
        for island in islands {
            hashset.insert(island);
        }
        hashset.len() as i32
    }

    fn find_all_islands(grid: &[Vec<i32>]) -> Vec<Vec<(usize, usize)>> {
        let mut res = vec![];
        let mut visited = HashSet::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != 1 {
                    continue;
                }
                if visited.contains(&(i, j)) {
                    continue;
                }
                let island = Self::dfs(grid, (i, j), &mut visited);
                res.push(island);
            }
        }
        res
    }

    fn dfs(
        grid: &[Vec<i32>],
        pos: (usize, usize),
        visited: &mut HashSet<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        let m = grid.len() - 1;
        let n = grid[0].len() - 1;

        let mut res = vec![];

        let mut stack = vec![];
        stack.push(pos);
        visited.insert(pos);
        while let Some((i, j)) = stack.pop() {
            res.push((i, j));

            if i > 0 && grid[i - 1][j] == 1 && !visited.contains(&(i - 1, j)) {
                stack.push((i - 1, j));
                visited.insert((i - 1, j));
            }
            if i < m && grid[i + 1][j] == 1 && !visited.contains(&(i + 1, j)) {
                stack.push((i + 1, j));
                visited.insert((i + 1, j));
            }
            if j > 0 && grid[i][j - 1] == 1 && !visited.contains(&(i, j - 1)) {
                stack.push((i, j - 1));
                visited.insert((i, j - 1));
            }
            if j < n && grid[i][j + 1] == 1 && !visited.contains(&(i, j + 1)) {
                stack.push((i, j + 1));
                visited.insert((i, j + 1));
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = Solution::num_distinct_islands(vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 1],
            vec![0, 0, 0, 1, 1],
        ]);
        assert_eq!(res, 1);
    }
}
