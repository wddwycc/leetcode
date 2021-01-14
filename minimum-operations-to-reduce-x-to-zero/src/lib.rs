pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut res = None;
        Self::dfs(&nums, x, 0, &mut res);
        res.map(|a| a as i32).unwrap_or(-1)
    }

    fn dfs(nums: &[i32], target: i32, steps: usize, min: &mut Option<usize>) {
        if target < 0 {
            return;
        } else if target == 0 {
            *min = Some(min.map(|a| std::cmp::min(steps, a)).unwrap_or(steps));
        } else {
            let len = nums.len();
            if len == 0 {
                return;
            } else if len == 1 {
                return Self::dfs(&nums[1..], target - nums[0], steps + 1, min);
            } else {
                Self::dfs(&nums[1..], target - nums[0], steps + 1, min);
                Self::dfs(&nums[0..(len - 1)], target - nums[len - 1], steps + 1, min);
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
