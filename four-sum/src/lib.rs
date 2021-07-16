pub struct Solution;
impl Solution {
    // time: O(n^3)
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        if n < 4 {
            return vec![];
        }
        nums.sort();

        let mut ans = vec![];
        for i in 0..(n - 3) {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in (i + 1)..(n - 2) {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                // two-cursor
                let target = target - nums[i] - nums[j];
                let mut l = j + 1;
                let mut r = n - 1;
                while l < r {
                    if nums[l] + nums[r] == target {
                        ans.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                        l += 1;
                        r -= 1;
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r + 1] {
                            r -= 1;
                        }
                    }
                    if nums[l] + nums[r] > target {
                        r -= 1;
                    }
                    if nums[l] + nums[r] < target {
                        l += 1;
                    }
                }
            }
        }
        ans
    }
}
