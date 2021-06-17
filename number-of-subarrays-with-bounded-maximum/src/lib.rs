pub struct Solution;
impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        // subarray num with max less than right + 1
        // minus
        // subarray num with max less than left
        Self::num_subarray_lt(&nums, right + 1) - Self::num_subarray_lt(&nums, left)
    }

    fn num_subarray_lt(nums: &[i32], bound: i32) -> i32 {
        let mut ans = 0;
        let mut acc = 0;
        for &num in nums {
            if num < bound {
                acc += 1;
                ans += acc;
            } else {
                acc = 0;
            }
        }
        ans
    }
}
