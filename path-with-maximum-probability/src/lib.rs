use std::collections::BinaryHeap;

#[derive(PartialEq, PartialOrd)]
struct NonNanF64(f64);
impl Eq for NonNanF64 {}
impl Ord for NonNanF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

pub struct Solution;
impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let n = n as usize;
        let start = start as usize;
        let end = end as usize;
        let adjacency_list = {
            let mut res = vec![vec![]; n];
            for (edge, prob) in edges.into_iter().zip(succ_prob) {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                res[u].push((v, prob));
                res[v].push((u, prob));
            }
            res
        };
        // dijkastra
        let mut prob_from_start = vec![0.0; n];
        prob_from_start[start] = 1.0;
        let mut pq: BinaryHeap<(NonNanF64, usize)> = BinaryHeap::new();
        pq.push((NonNanF64(1.0), start));
        while let Some((NonNanF64(prob), u)) = pq.pop() {
            if u == end {
                return prob;
            }
            for &(v, to_v_prob) in &adjacency_list[u] {
                let next_prob = prob * to_v_prob;
                if next_prob > prob_from_start[v] {
                    prob_from_start[v] = next_prob;
                    pq.push((NonNanF64(next_prob), v));
                }
            }
        }
        0.0
    }
}
