use std::collections::{HashSet, VecDeque};

type Code = (usize, usize, usize, usize);

pub struct Solution;
impl Solution {
    fn parse_code(src: String) -> Code {
        let mut iter = src
            .chars()
            .into_iter()
            .map(|a| a.to_string().parse::<usize>().unwrap());
        (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    }

    fn move_up(src: usize) -> usize {
        if src == 9 {
            0
        } else {
            src + 1
        }
    }

    fn move_down(src: usize) -> usize {
        if src == 0 {
            9
        } else {
            src - 1
        }
    }

    fn moves(src: Code) -> Vec<Code> {
        vec![
            (Self::move_up(src.0), src.1, src.2, src.3),
            (Self::move_down(src.0), src.1, src.2, src.3),
            (src.0, Self::move_up(src.1), src.2, src.3),
            (src.0, Self::move_down(src.1), src.2, src.3),
            (src.0, src.1, Self::move_up(src.2), src.3),
            (src.0, src.1, Self::move_down(src.2), src.3),
            (src.0, src.1, src.2, Self::move_up(src.3)),
            (src.0, src.1, src.2, Self::move_down(src.3)),
        ]
    }

    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let deadends: HashSet<Code> = deadends.into_iter().map(Self::parse_code).collect();
        let target = Self::parse_code(target);

        let mut visited = HashSet::new();
        let mut step = 0;
        let mut queue: VecDeque<Code> = VecDeque::new();
        queue.push_back((0, 0, 0, 0));
        loop {
            let queue_len = queue.len();
            if queue_len == 0 {
                return -1;
            }
            for _ in 0..queue_len {
                let root = queue.pop_front().unwrap();
                if deadends.contains(&root) {
                    return -1;
                }
                if root == target {
                    return step;
                }
                for next in Self::moves(root) {
                    if visited.contains(&next) || deadends.contains(&next) {
                        continue;
                    }
                    visited.insert(next);
                    queue.push_back(next);
                }
            }
            step += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::parse_code("0000".to_string()), (0, 0, 0, 0));
    }
}
