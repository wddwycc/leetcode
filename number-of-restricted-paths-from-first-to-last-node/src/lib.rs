use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct Solution;
impl Solution {
    pub fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let adjancency_list = {
            let mut res = vec![vec![]; n + 1];
            for edge in edges {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                let w = edge[2];
                res[u].push((v, w));
                res[v].push((u, w));
            }
            res
        };
        // Step1: use dijkstra to calc all shortest path from i to n.
        let mut distances = vec![std::i32::MAX; n + 1];
        distances[n] = 0;
        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0), n));
        while let Some((Reverse(distance), u)) = pq.pop() {
            for &(v, w) in &adjancency_list[u] {
                let next_distance = distance + w;
                if next_distance < distances[v] {
                    distances[v] = next_distance;
                    pq.push((Reverse(next_distance), v));
                }
            }
        }
        // Step2: use top-down dp to calc numbers of paths
        let mut cache = HashMap::new();
        Self::dfs(&adjancency_list, &distances, 1, &mut cache) as i32
    }

    fn dfs(
        adjancency_list: &[Vec<(usize, i32)>],
        distances: &[i32],
        u: usize,
        cache: &mut HashMap<usize, i64>,
    ) -> i64 {
        if let Some(&cached) = cache.get(&u) {
            return cached;
        }
        let mut res = 0;
        for &(v, _) in &adjancency_list[u] {
            if distances[v] == 0 {
                res += 1;
            } else if distances[u] > distances[v] {
                res += Self::dfs(adjancency_list, distances, v, cache);
            }
        }
        res %= 1_000_000_007;
        cache.insert(u, res);
        res
    }
}
