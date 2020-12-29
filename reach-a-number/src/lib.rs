pub struct Solution;
impl Solution {
    /// Observation
    /// 1. Steps(n) = Steps(-n)
    /// 2. +/- 1 +/- 2 +/- 3 .... +/- k = S
    ///
    /// To find smallest k, we all use +
    /// 1 + 2 + 3 + ..... + k = target + d, (0 <= d < k)
    ///
    /// 1. if d == 0: k is the answer
    /// 2. if d % 2 == 0: 1 + 2 + 3 + ... - i + ... + k = target, i = d / 2
    /// 3. if d % 2 == 1:
    ///     * if k % 2 == 0, 1 + 2 + 3 + ... - i + ... + k + (k + 1) = target,
    ///         i = ((k + 1) + d) / 2 <= k
    ///     * if k % 2 == 1, 1 + 2 + 3 + ... - i + ... + k + (k + 1) + (k + 2) = target,
    ///         i = ((k + 2) - (k + 1) + d) / 2 = (d + 1) / 2 <= k
    pub fn reach_number(target: i32) -> i32 {
        let target = target.abs();
        let k = (((1f32 + 8f32 * target as f32).sqrt() - 1f32) / 2f32).ceil() as i32;
        let s = (1 + k) * k / 2;
        let d = s - target;
        if d % 2 == 0 {
            k
        } else {
            if (k + 1) % 2 == 0 {
                k + 2
            } else {
                k + 1
            }
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
