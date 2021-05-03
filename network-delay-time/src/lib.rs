use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let adjacency_list = {
            let mut res = vec![vec![]; n + 1];
            for time in times {
                let u = time[0] as usize;
                let v = time[1] as usize;
                let w = time[2];
                res[u].push((v, w));
            }
            res
        };

        // dijkstra
        let mut distances = vec![std::i32::MAX; n + 1];
        distances[k] = 0;
        let mut visited = vec![false; n + 1];
        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0), k));
        while let Some((Reverse(distance), u)) = pq.pop() {
            if visited[u] {
                continue;
            }
            for &(v, w) in &adjacency_list[u] {
                let new_distance = w + distance;
                if new_distance < distances[v] {
                    distances[v] = new_distance;
                    pq.push((Reverse(new_distance), v));
                }
            }
            visited[u] = true;
        }
        if !visited[1..].iter().all(|&a| a) {
            return -1;
        }
        *distances[1..].iter().max().unwrap()
    }
}
