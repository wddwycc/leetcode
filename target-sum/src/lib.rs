pub struct Solution;
impl Solution {
    pub fn find_target_sum_ways(mut nums: Vec<i32>, s: i32) -> i32 {
        // Brute force
        if let Some(num) = nums.pop() {
            return Self::find_target_sum_ways(nums.clone(), s - num)
                + Self::find_target_sum_ways(nums, s + num);
        } else {
            if s == 0 {
                return 1;
            } else {
                return 0;
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
