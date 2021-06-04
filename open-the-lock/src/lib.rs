use std::collections::{HashSet, VecDeque};

pub struct Solution;
impl Solution {
    // NOTE: use 4 bits per wheel to store pos, totally 16 bits
    fn bitmask_pos(pos: String) -> u16 {
        let mut res = 0;
        for b in pos.bytes() {
            res = (res << 4) + (b - b'0') as u16;
        }
        res
    }

    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let deadends = deadends
            .into_iter()
            .map(Self::bitmask_pos)
            .collect::<HashSet<_>>();
        let start = Self::bitmask_pos(target);
        // BFS
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);
        let mut steps = 0;
        while queue.len() > 0 {
            for _ in 0..queue.len() {
                let pos = queue.pop_front().unwrap();
                if pos == 0 {
                    return steps;
                }
                // try go every directions
                for offset in 0..4 {
                    let bit_offset = offset * 4;
                    let v = ((15 << bit_offset) & pos) >> bit_offset;
                    let next_pos1 = if v == 9 {
                        pos - (9 << bit_offset)
                    } else {
                        pos + (1 << bit_offset)
                    };
                    let next_pos2 = if v == 0 {
                        pos + (9 << bit_offset)
                    } else {
                        pos - (1 << bit_offset)
                    };
                    if !visited.contains(&next_pos1) && !deadends.contains(&next_pos1) {
                        visited.insert(next_pos1);
                        queue.push_back(next_pos1)
                    }
                    if !visited.contains(&next_pos2) && !deadends.contains(&next_pos2) {
                        visited.insert(next_pos2);
                        queue.push_back(next_pos2);
                    }
                }
            }
            steps += 1;
        }
        -1
    }
}
