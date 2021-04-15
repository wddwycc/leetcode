pub struct Solution {}
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let mut dp = (0, 1);
        for _ in 2..=(n as usize) {
            let res = dp.0 + dp.1;
            dp.0 = dp.1;
            dp.1 = res;
        }
        dp.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
    }
}
