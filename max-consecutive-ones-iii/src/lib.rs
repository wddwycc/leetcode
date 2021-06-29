pub struct Solution;
impl Solution {
    // sliding window, time: O(n)
    pub fn longest_ones(a: Vec<i32>, mut k: i32) -> i32 {
        let n = a.len();
        let mut l = 0;
        for r in 0..n {
            if a[r] == 0 {
                k -= 1;
            }
            if k < 0 {
                if a[l] == 0 {
                    k += 1;
                }
                l += 1;
            }
        }
        (n - l) as i32
    }
}
