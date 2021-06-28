pub struct Solution
impl Solution {
    // O(n^2)
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let n = nums.len();
        let max = nums[n - 1] - nums[0];
        let mut buckets = vec![0; max as usize + 1];
        for i in 0..(n - 1) {
            for j in (i + 1)..n {
                let idx = (nums[i] - nums[j]).abs() as usize;
                buckets[idx] += 1;
            }
        }
        let mut k = k as usize;
        let mut cur = 0;
        loop {
            if buckets[cur] < k {
                k -= buckets[cur];
                cur += 1;
            } else {
                return cur as i32;   
            }
        }
    }
}