use std::collections::{HashMap, HashSet};

pub struct Solution;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut matrix = vec![vec![false; n]; n]; // adjacency matrix
        for row in prerequisites {
            matrix[row[0] as usize][row[1] as usize] = true;
        }
        // f(i) -> bool, represents course i can is learnable.
        // for a cource that is learnable, there are two cases
        //   1. it has no dependency
        //   2. all its' dependencies are learnable
        //
        // Translate into code:
        //   1. for j in 0..n, matrix[i][j] is false;
        //   2. for j in 0..n, when matrix[i][j] is true, f(j) must be true.

        // one last thing: we must detect circular dependency to avoid infinite recusion.

        // let's try bfs with memoization
        let mut cache = HashMap::new();
        for i in 0..n {
            let mut visited = HashSet::new();
            visited.insert(i);
            let res = Self::dfs(&matrix, n, i, &mut cache, &mut visited);
            if !res {
                return false;
            }
        }
        true
    }

    fn dfs(
        matrix: &[Vec<bool>],
        n: usize,
        i: usize,
        cache: &mut HashMap<usize, bool>,
        visited: &mut HashSet<usize>,
    ) -> bool {
        if let Some(cached) = cache.get(&i) {
            return *cached;
        }

        let res = (|| {
            for j in 0..n {
                if !matrix[i][j] {
                    continue;
                }
                if visited.contains(&j) {
                    return false;
                }
                let mut new_visited = visited.clone();
                new_visited.insert(j);
                if !Self::dfs(matrix, n, j, cache, &mut new_visited) {
                    return false;
                }
            }
            return true;
        })();
        cache.insert(i, res);
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
