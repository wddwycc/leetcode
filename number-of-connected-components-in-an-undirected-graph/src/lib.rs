use std::collections::{HashSet, VecDeque};

pub struct Solution;
impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let adjacency_matrix = {
            let mut res = vec![vec![false; n]; n];
            for edge in edges {
                res[edge[0] as usize][edge[1] as usize] = true;
                res[edge[1] as usize][edge[0] as usize] = true;
            }
            res
        };
        // NOTE: BFS on every node
        let mut res = 0;
        let mut visited = HashSet::new();
        for i in 0..n {
            if visited.contains(&i) {
                continue;
            }
            res += 1;
            let mut queue = VecDeque::new();
            queue.push_back(i);
            while let Some(node) = queue.pop_front() {
                visited.insert(node);
                for (idx, &b) in adjacency_matrix[node].iter().enumerate() {
                    if !b {
                        continue;
                    }
                    if visited.contains(&idx) {
                        continue;
                    }
                    queue.push_back(idx);
                }
            }
        }
        res
    }
}
