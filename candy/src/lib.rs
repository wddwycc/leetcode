pub struct Solution;
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let l2r = {
            let mut res = vec![1; n];
            let mut acc = 0;
            for i in 1..n {
                if ratings[i] <= ratings[i - 1] {
                    acc = 0;
                    continue;
                }
                acc += 1;
                res[i] += acc;
            }
            res
        };
        let r2l = {
            let mut res = vec![1; n];
            let mut acc = 0;
            for i in (0..(n - 1)).rev() {
                if ratings[i] <= ratings[i + 1] {
                    acc = 0;
                    continue;
                }
                acc += 1;
                res[i] += acc;
            }
            res
        };
        let mut ans = 0;
        for i in 0..n {
            ans += std::cmp::max(l2r[i], r2l[i]);
        }
        ans
    }
}
