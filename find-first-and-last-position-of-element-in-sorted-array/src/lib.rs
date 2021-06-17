pub struct Solution;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // find the value
        let pivot = match nums.binary_search(&target) {
            Ok(a) => a,
            _ => return vec![-1, -1],
        };
        // seek left
        let left = {
            let mut l = 0;
            let mut r = pivot;
            while l < r {
                let mid = l + (r - l) / 2;
                if nums[mid] == target {
                    r = mid;
                } else {
                    l = mid + 1;
                }
            }
            l as i32
        };
        // seek right
        let right = if pivot == nums.len() - 1 {
            pivot as i32
        } else {
            let mut l = pivot;
            let mut r = nums.len() - 1;
            while l < r {
                let mid = l + (r - l) / 2;
                if nums[mid] == target {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }
            r as i32 - 1
        };
        vec![left, right]
    }
}
