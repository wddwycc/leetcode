pub struct Solution;
impl Solution {
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let m = a.len();
        let n = b.len();

        let mut ans = 0;
        let mut dp = vec![vec![0; n]; m];
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let mut res = 0;
                if a[i] == b[j] {
                    res += 1;
                    if i + 1 < m && j + 1 < n {
                        res += dp[i + 1][j + 1];
                    }
                }
                dp[i][j] = res;
                ans = ans.max(res);
            }
        }
        ans
    }
}
