// O(n^2), brute-force, calc all distances and enumerate with bucket sorting
pub struct Solution1;
impl Solution1 {
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

pub struct Solution2;
impl Solution2 {
    // O(nlogn)
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let n = nums.len();
        let k = k as usize;

        // given function f(i) = ct (nums of pairs with distance lte i)
        // binary search i range from [0, max_distance]
        // find first i that satisfies f(i) >= k
        let mut l = 0;
        let mut r = nums[n - 1] - nums[0];

        while l < r {
            let mid = l + (r - l) / 2;
            let mut ct = 0;
            for i in 0..(n - 1) {
                let fst_v_gte_nums_i = nums[i] + mid + 1;
                match &nums[(i + 1)..].binary_search(&fst_v_gte_nums_i) {
                    Ok(j) => ct += j,
                    Err(j) => ct += j,
                }
            }
            if ct < k {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l
    }
}
