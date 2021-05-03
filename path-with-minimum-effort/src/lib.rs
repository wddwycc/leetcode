use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;
impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        // dijkstra
        let m = heights.len();
        let n = heights[0].len();

        let mut min_effort = vec![vec![std::i32::MAX; n]; m];
        let mut pq: BinaryHeap<(Reverse<i32>, (usize, usize), Option<(usize, usize)>)> =
            BinaryHeap::new();
        pq.push((Reverse(0), (0, 0), None));
        while let Some((Reverse(from_effort), pos, from_pos)) = pq.pop() {
            let (x, y) = pos;
            if x + 1 == m && y + 1 == n {
                return from_effort;
            }
            let height = heights[x][y];
            let mut seek = |to_pos: (usize, usize)| {
                if Some(to_pos) == from_pos {
                    return;
                }
                let mut effort = (heights[to_pos.0][to_pos.1] - height).abs();
                effort = effort.max(from_effort);
                if effort < min_effort[to_pos.0][to_pos.1] {
                    min_effort[to_pos.0][to_pos.1] = effort;
                    pq.push((Reverse(effort), to_pos, from_pos));
                }
            };
            if x > 0 {
                seek((x - 1, y));
            }
            if x + 1 < m {
                seek((x + 1, y));
            }
            if y > 0 {
                seek((x, y - 1));
            }
            if y + 1 < n {
                seek((x, y + 1));
            }
        }
        panic!();
    }
}
