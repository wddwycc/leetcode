use std::collections::{HashSet, VecDeque};

pub struct Solution;
impl Solution {
    pub fn min_knight_moves(x: i32, y: i32) -> i32 {
        // BFS Search
        let mut visited = HashSet::new();
        let mut deque = VecDeque::new();
        deque.push_back((x, y));
        let mut steps = 0;
        while deque.len() > 0 {
            for _ in 0..deque.len() {
                let pos = deque.pop_front().unwrap();
                if pos == (0, 0) {
                    return steps;
                }
                for i in [-1, 1].iter() {
                    for j in [-2, 2].iter() {
                        for next_pos in [(pos.0 + i, pos.1 + j), (pos.0 + j, pos.1 + i)].iter() {
                            if visited.contains(next_pos) {
                                continue;
                            }
                            visited.insert(*next_pos);
                            deque.push_back(*next_pos);
                        }
                    }
                }
            }
            steps += 1;
        }
        panic!()
    }
}
