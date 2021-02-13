use std::collections::{HashSet, VecDeque};

pub struct Solution;
impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len() - 1;
        let n = grid[0].len() - 1;
        if grid[m][n] != 0 || grid[0][0] != 0 {
            return -1;
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut level = 1;
        queue.push_back((0, 0));
        while queue.len() > 0 {
            for _ in 0..queue.len() {
                let (x, y) = queue.pop_front().unwrap();
                if visited.contains(&(x, y)) {
                    continue;
                }
                if x == m && y == n {
                    return level;
                }
                visited.insert((x, y));
                for (x, y) in Self::next_pos((x, y), m, n) {
                    if !visited.contains(&(x, y)) && grid[x][y] == 0 {
                        queue.push_back((x, y));
                    }
                }
            }
            level += 1;
        }
        -1
    }

    fn next_pos(from: (usize, usize), m: usize, n: usize) -> Vec<(usize, usize)> {
        let mut res = vec![];
        let (x, y) = from;
        if x > 0 {
            res.push((x - 1, y));
        }
        if x < m {
            res.push((x + 1, y));
        }
        if y > 0 {
            res.push((x, y - 1));
        }
        if y < n {
            res.push((x, y + 1));
        }
        if x > 0 && y > 0 {
            res.push((x - 1, y - 1));
        }
        if x > 0 && y < n {
            res.push((x - 1, y + 1));
        }
        if x < m && y > 0 {
            res.push((x + 1, y - 1));
        }
        if x < m && y < n {
            res.push((x + 1, y + 1));
        }
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
