pub struct Solution;
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let target = n;
        let max_el = (n as f64).sqrt() as i32;
        let els = (1..=max_el).map(|a| a * a).collect::<Vec<i32>>();

        // NOTE: BFS
        // VecDeque<(last_val, target_distance)>
        let mut queue = std::collections::VecDeque::new();
        for el in &els {
            queue.push_back((el, target - el));
        }
        let mut steps = 1;
        loop {
            let q_len = queue.len();
            if q_len == 0 {
                return -1;
            }
            for _ in 0..q_len {
                let (last_val, target_distance) = queue.pop_front().unwrap();
                if target_distance == 0 {
                    return steps;
                }
                if target_distance < 0 {
                    continue;
                }
                for a in &els {
                    if a <= last_val {
                        queue.push_back((a, target_distance - a))
                    }
                }
            }
            steps += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
