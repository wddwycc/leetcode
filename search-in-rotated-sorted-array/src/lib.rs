pub struct Solution;
impl Solution {
    // O(nlogn)
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l <= r {
            let mid = l + (r - l) / 2;
            // case 1: hit
            if nums[mid] == target {
                return mid as i32;
            }
            // case 2: mid is in the rotated section
            if nums[mid] <= nums[r] {
                if target > nums[mid] && target <= nums[r] {
                    l = mid + 1;
                } else {
                    if mid >= 1 {
                        r = mid - 1;
                    } else {
                        return -1;
                    }
                }
            }
            // case 3: mid is in the non-rotated section
            else {
                if target < nums[mid] && target >= nums[l] {
                    if mid >= 1 {
                        r = mid - 1;
                    } else {
                        return -1;
                    }
                } else {
                    l = mid + 1;
                }
            }
        }
        -1
    }
}
