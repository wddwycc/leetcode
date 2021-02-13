pub struct Solution;
impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % k != 0 {
            return false;
        }
        let targer_sum = sum / k;
        let k = k as usize;

        let mut visited = vec![false; nums.len()];
        Self::dfs(&nums, k, targer_sum, 0, 0, 0, &mut visited)
    }

    fn dfs(
        // consts
        nums: &[i32],
        k: usize,
        target_sum: i32,
        // vars
        cur: usize,
        group_cur: usize,
        sum: i32,
        visited: &mut Vec<bool>,
    ) -> bool {
        if group_cur == k {
            return true;
        }
        if sum > target_sum {
            return false;
        }
        if sum == target_sum {
            return Self::dfs(nums, k, target_sum, 0, group_cur + 1, 0, visited);
        }
        for i in cur..nums.len() {
            if visited[i] {
                continue;
            }
            visited[i] = true;
            if Self::dfs(
                nums,
                k,
                target_sum,
                i + 1,
                group_cur,
                sum + nums[i],
                visited,
            ) {
                return true;
            }
            visited[i] = false;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = Solution::can_partition_k_subsets(vec![10, 10, 10, 7, 7, 7, 7, 7, 7, 6, 6, 6], 3);
        assert_eq!(res, true);
    }
}
