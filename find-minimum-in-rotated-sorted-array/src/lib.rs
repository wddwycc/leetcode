pub struct Solution;
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let first_el = nums[0];
        // find the decrease pivot
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            let mid = l + (r - l) / 2;
            if nums[mid] >= first_el {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        if nums[l] > first_el {
            first_el
        } else {
            nums[l]
        }
    }
}
