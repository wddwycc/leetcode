pub struct Solution;
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut slots: Vec<(i32, i32, i32)> = start_time
            .into_iter()
            .zip(end_time.into_iter())
            .zip(profit.into_iter())
            .map(|((s, e), p)| (s, e, p))
            .collect();
        slots.sort_by(|x, y| x.0.cmp(&y.0));
        Self::dfs(&slots)
    }

    fn dfs(slots: &[(i32, i32, i32)]) -> i32 {
        if slots.len() == 0 {
            return 0;
        }
        // for every time slot, one can choose do or not.
        // if do the job, get the profit, to the next start_time, dfs again
        // else, dfs next time slot directly

        // NOTE: consume first el
        let res1 = {
            let mut res = slots[0].2;
            for i in 1..slots.len() {
                if slots[i].0 >= slots[0].1 {
                    res += Self::dfs(&slots[i..]);
                    break;
                }
            }
            res
        };
        // NOTE: skip first el
        let res2 = Self::dfs(&slots[1..]);

        std::cmp::max(res1, res2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
