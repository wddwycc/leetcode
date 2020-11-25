use std::vec;

pub struct Solution;
impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut empties: Vec<(usize, usize)> = vec![];
        // (start idx, len)
        let mut tmp: Option<(usize, usize)> = None;
        for idx in 0..seats.len() {
            if seats[idx] == 0 {
                match tmp.as_mut() {
                    Some(a) => a.1 = idx,
                    None => tmp = Some((idx, idx)),
                }
            } else {
                if let Some(t) = tmp {
                    empties.push(t);
                    tmp = None;
                }
            }
        }
        if let Some(t) = tmp {
            empties.push(t);
        }
        empties
            .iter()
            .map(|(l, r)| {
                if *l == 0 || *r == seats.len() - 1 {
                    return r - l;
                }
                return (r - l) / 2;
            })
            .max()
            .unwrap() as i32
            + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1]), 2);
        assert_eq!(Solution::max_dist_to_closest(vec![1, 0, 0, 0]), 3);
    }
}
