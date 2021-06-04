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
        let target = Self::bitmask_pos(target);
        let mut visited = deadends
            .into_iter()
            .map(Self::bitmask_pos)
            .collect::<HashSet<_>>();
        let mut queue = VecDeque::new();
        queue.push_back(0);
        let mut steps = 0;
        while queue.len() > 0 {
            for _ in 0..queue.len() {
                let pos = queue.pop_front().unwrap();
                if visited.contains(&pos) {
                    continue;
                }
                if pos == target {
                    return steps;
                }
                for offset in 0..4 {
                    let bit_offset = offset * 4;
                    let v = ((15 << bit_offset) & pos) >> bit_offset;
                    for &next_pos in &[
                        if v == 9 {
                            pos - (9 << bit_offset)
                        } else {
                            pos + (1 << bit_offset)
                        },
                        if v == 0 {
                            pos + (9 << bit_offset)
                        } else {
                            pos - (1 << bit_offset)
                        },
                    ] {
                        if next_pos == target {
                            return steps + 1;
                        }
                        queue.push_back(next_pos);
                    }
                }
                visited.insert(pos);
            }
            steps += 1;
        }
        -1
    }
}
