pub struct Solution;
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|x, y| x[0].cmp(&y[0]));

        let mut res = vec![];
        for interval in intervals {
            let prev_interval = match res.last_mut() {
                Some(a) => a,
                None => {
                    res.push(interval);
                    continue;
                }
            };
            if prev_interval[1] >= interval[0] {
                // NOTE: overlaps
                *prev_interval = vec![
                    prev_interval[0],
                    std::cmp::max(interval[1], prev_interval[1]),
                ];
            } else {
                res.push(interval);
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
