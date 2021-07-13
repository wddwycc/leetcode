pub struct Solution;
impl Solution {
    // two rules used:
    // 1: (a ^ b) ^ c = a ^ (b * c)
    // 2: (a * b) % k = (a % k) * (b % k)

    // reduce into subproblem:
    // a ^ 1234567
    // = (a ^ 1234560) * (a ^ 7)
    // = ((a ^ 123456) ^ 10) * (a ^ 7)
    pub fn super_pow(a: i32, mut b: Vec<i32>) -> i32 {
        let last_digit = match b.pop() {
            Some(a) => a,
            None => return 1,
        };
        let base = 1337;
        let a = a % base;
        let mut res = 1;
        let mut prev_pow = Self::super_pow(a, b);
        for _ in 0..10 {
            res *= prev_pow;
            res %= base;
        }
        for _ in 0..last_digit {
            res *= a;
            res %= base;
        }
        res %= base;
        res
    }
}
