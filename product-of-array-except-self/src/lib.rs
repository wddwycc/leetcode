pub struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![1; n];
        let mut acc = 1;
        for i in 1..n {
            acc *= nums[i - 1];
            ans[i] *= acc;
        }
        let mut acc = 1;
        for i in (0..(n - 1)).rev() {
            acc *= nums[i + 1];
            ans[i] *= acc;
        }
        ans
    }
}
