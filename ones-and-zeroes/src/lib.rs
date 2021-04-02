pub struct Solution;
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize + 1]; m as usize + 1];
        for str in strs {
            let (zeros, ones) = Self::count_zero_ones(str);
            for i in (zeros..=(m as usize)).rev() {
                for j in (ones..=(n as usize)).rev() {
                    dp[i][j] = dp[i][j].max(1 + dp[i - zeros][j - ones]);
                }
            }
        }
        dp[m as usize][n as usize]
    }

    fn count_zero_ones(str: String) -> (usize, usize) {
        let mut zeros = 0;
        let mut ones = 0;
        for b in str.bytes() {
            match b {
                b'0' => zeros += 1,
                b'1' => ones += 1,
                _ => panic!(),
            }
        }
        (zeros, ones)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
