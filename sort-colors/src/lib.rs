pub struct Solution;
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = nums.len() as i32 - 1;
        while j <= k {
            if nums[j as usize] == 0 {
                nums.swap(i as usize, j as usize);
                i += 1;
                j += 1;
            } else if nums[j as usize] == 2 {
                nums.swap(j as usize, k as usize);
                k -= 1;
            } else {
                j += 1;
            }
        }
    }
}
