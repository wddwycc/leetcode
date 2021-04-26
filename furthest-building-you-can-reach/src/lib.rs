use std::collections::BinaryHeap;

pub struct Solution;
impl Solution {
    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
        let mut bricks_pq: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        let mut ladders_pq: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        let mut cur = 0;
        while cur < heights.len() {
            // Note: is last
            if cur + 1 == heights.len() {
                return cur as i32;
            }
            // Note: can move directly
            let offset = heights[cur + 1] - heights[cur];
            if offset <= 0 {
                cur += 1;
                continue;
            }
            if bricks >= offset {
                // Note: use bricks when possible
                bricks -= offset;
                bricks_pq.push((offset, cur));
                cur += 1;
            } else if ladders > 0 {
                // Note: use ladder when possible
                ladders -= 1;
                ladders_pq.push((offset, cur));
                cur += 1;
            } else {
                // Note: replace ladders internally
                if let Some(&a) = ladders_pq.peek() {
                    if let Some(&b) = bricks_pq.peek() {
                        if a.0 < b.0 {
                            bricks += b.0 - a.0;
                            ladders_pq.pop();
                            bricks_pq.pop();
                            ladders_pq.push(b);
                            bricks_pq.push(a);
                            continue;
                        }
                    }
                }
                return cur as i32;
            }
        }
        panic!()
    }
}
