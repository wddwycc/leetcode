pub struct Solution;
impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3 {
            return 0;
        }
        nums.sort();

        let mut ans = 0;
        for i in 0..(n - 2) {
            for j in (i + 1)..(n - 1) {
                let sum = nums[i] + nums[j];
                // from close range [j + 1, n - 1]
                // find first el that gte sum
                let mut l = j + 1;
                let mut r = n - 1;
                while l < r {
                    let mid = l + (r - l) / 2;
                    if nums[mid] >= sum {
                        r = mid;
                    } else {
                        l = mid + 1;
                    }
                }
                if l == n - 1 && nums[l] < sum {
                    l += 1;
                }
                ans += l - (j + 1);
            }
        }
        ans as i32
    }
}
