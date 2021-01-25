use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let target: i32 = nums.iter().sum();
        if target % 2 != 0 { return false }
        let target = target / 2;

        // Trasforms the question into:
        // Is it possible for subset of nums to sum up to target
        
        // If enumerate all subsets, time would be O(2 ** n). as n can reach 200, would cause time exceed.
        
        // Try use top-down dp
        // The state to memorize is:
        // for nums[i..], whether there exists a subset sums up to j
        // represented as HashMap<(i, j), bool>
        let mut cache = HashMap::new();
        Self::dfs(&nums, 0, target, &mut cache)
    }
    
    pub fn dfs(nums: &[i32], index: usize, sum: i32, cache: &mut HashMap<(usize, i32), bool>) -> bool {
        if let Some(cached) = cache.get(&(index, sum)) {
            return *cached;
        }
        if sum < 0 {
            return false;
        }
        if sum == 0 {
            return true;
        }
        if index == nums.len() {
            return false;
        }
        let res = Self::dfs(nums, index + 1, sum, cache) || Self::dfs(nums, index + 1, sum - nums[index], cache);
        cache.insert((index, sum), res);
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
