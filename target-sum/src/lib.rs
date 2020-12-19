pub struct Solution;
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        Self::helper(&nums, s)
    }

    pub fn helper(nums: &[i32], s: i32) -> i32 {
        if let Some(num) = nums.last() {
            Self::helper(&nums[0..(nums.len() - 1)], s - num)
                + Self::helper(&nums[0..(nums.len() - 1)], s + num)
        } else {
            if s == 0 {
                1
            } else {
                0
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
