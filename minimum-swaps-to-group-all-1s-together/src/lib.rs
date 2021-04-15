pub struct Solution;
impl Solution {
    pub fn min_swaps(data: Vec<i32>) -> i32 {
        let n = data.len();
        let k = data.iter().filter(|&&a| a == 1).count();

        if k == 0 || k == 1 {
            return 0;
        }

        let mut first_is_zero = data[0] == 0;
        let mut dp = vec![(0..k).filter(|&i| data[i] == 0).count() as i32];

        for i in 1..=(n - k) {
            let mut next_v = dp[i - 1];
            if first_is_zero {
                next_v -= 1;
            }
            if data[i + k - 1] == 0 {
                next_v += 1;
            }
            first_is_zero = data[i] == 0;
            dp.push(next_v);
        }
        dp.into_iter().min().unwrap()
    }
}
