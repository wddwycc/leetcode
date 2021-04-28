use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;
impl Solution {
    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, ladders: i32) -> i32 {
        let mut pq = BinaryHeap::new();
        for i in 1..heights.len() {
            let offset = heights[i] - heights[i - 1];
            if offset <= 0 {
                continue;
            }
            pq.push(Reverse(offset));
            if pq.len() > ladders as usize {
                let Reverse(to_replace) = pq.pop().unwrap();
                if to_replace <= bricks {
                    bricks -= to_replace;
                } else {
                    return i as i32 - 1;
                }
            }
        }
        heights.len() as i32 - 1
    }
}
