use std::collections::{HashSet, VecDeque};

pub struct Solution;
impl Solution {
    pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = matrix.len();
        let columns = matrix[0].len();

        let mut res = vec![vec![0; columns]; rows];
        // do BFS for each node
        for r in 0..rows {
            for c in 0..columns {
                res[r][c] = Self::bfs(&matrix, (r, c), rows, columns) as i32;
            }
        }
        res
    }

    fn bfs(src: &[Vec<i32>], pos: (usize, usize), rows: usize, columns: usize) -> usize {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(pos);
        let mut level = 0;

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let v = queue.pop_front().unwrap();
                if src[v.0][v.1] == 0 {
                    return level;
                }
                visited.insert(v);
                let mut enqueue_if_needed = |pos: (usize, usize)| {
                    if visited.contains(&pos) {
                        return;
                    }
                    queue.push_back(pos);
                };
                if v.0 > 0 {
                    enqueue_if_needed((v.0 - 1, v.1));
                }
                if v.0 < rows - 1 {
                    enqueue_if_needed((v.0 + 1, v.1));
                }
                if v.1 > 0 {
                    enqueue_if_needed((v.0, v.1 - 1));
                }
                if v.1 < columns - 1 {
                    enqueue_if_needed((v.0, v.1 + 1));
                }
            }
            level += 1;
        }
        panic!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
