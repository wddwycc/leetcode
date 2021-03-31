pub struct Solution;
impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let stamp = stamp.into_bytes();
        let mut target = target.into_bytes();

        let mut res = vec![];
        let mut visited = vec![false; target.len()];
        let mut stars = 0;

        while stars < target.len() {
            let mut done_replace = false;
            for i in 0..=(target.len() - stamp.len()) {
                if !visited[i] && Self::can_replace(&target, &stamp, i) {
                    stars += Self::do_replace(&mut target, stamp.len(), i);
                    done_replace = true;
                    visited[i] = true;
                    res.push(i as i32);
                    if stars == target.len() {
                        break;
                    }
                }
            }
            if !done_replace {
                return vec![0];
            }
        }
        res.reverse();
        res
    }

    fn can_replace(target: &[u8], stamp: &[u8], cur: usize) -> bool {
        for i in 0..stamp.len() {
            if target[cur + i] != ('*' as u8) && target[cur + i] != stamp[i] {
                return false;
            }
        }
        true
    }

    fn do_replace(target: &mut [u8], s_len: usize, cur: usize) -> usize {
        let mut res = 0;
        for i in 0..s_len {
            if target[cur + i] != '*' as u8 {
                target[cur + i] = '*' as u8;
                res += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
