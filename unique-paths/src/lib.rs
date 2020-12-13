pub struct Solution;
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut res = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                match (i, j) {
                    (0, 0) => res[i][j] = 1,
                    (0, j) => res[i][j] = res[i][j - 1],
                    (i, 0) => res[i][j] = res[i - 1][j],
                    (i, j) => res[i][j] = res[i - 1][j] + res[i][j - 1],
                }
            }
        }
        res[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
