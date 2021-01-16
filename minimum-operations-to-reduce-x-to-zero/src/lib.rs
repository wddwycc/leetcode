pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let len = nums.len();
        let total: i32 = nums.iter().sum();
        let target = total - x;
        // NOTE: transform the question into: find longest subarray to sum up to target
        let mut res: i32 = -1;
        let mut current_v = 0;
        let mut left = 0;
        for right in 0..len {
            current_v += nums[right];
            while current_v > target && left <= right {
                current_v -= nums[left];
                left += 1;
            }
            if current_v == target {
                res = std::cmp::max(res, (right - left + 1) as i32);
            }
        }
        if res == -1 {
            return -1;
        } else {
            return len as i32 - res;
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
