pub struct Solution;
impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut l = 0;
        let mut r = 0;
        let mut cost = 0;
        // sliding window
        Self::move_r_to_limit(&nums, k, &mut cost, l, &mut r);
        let mut res = r - l + 1;
        while l + 1 < nums.len() {
            cost -= nums[r] - nums[l];
            l += 1;
            Self::move_r_to_limit(&nums, k, &mut cost, l, &mut r);
            res = res.max(r - l + 1);
        }
        res as i32
    }

    fn move_r_to_limit(nums: &[i32], k: i32, cost: &mut i32, l: usize, r: &mut usize) {
        while *r + 1 < nums.len() {
            let offset = (*r as i32 - l as i32 + 1) * (nums[*r + 1] - nums[*r]);
            if *cost + offset <= k {
                *r += 1;
                *cost += offset;
            } else {
                break;
            }
        }
    }
}
