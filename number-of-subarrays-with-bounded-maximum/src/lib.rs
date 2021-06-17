pub struct Solution;
impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        // brute-force
        let n = nums.len();
        let mut ans = 0;
        let mut max = 0;
        for i in 0..n {
            for j in i..n {
                max = if i == j { nums[j] } else { max.max(nums[j]) };
                if max > right {
                    break;
                }
                if max >= left {
                    ans += 1;
                }
            }
        }
        ans
    }
}
