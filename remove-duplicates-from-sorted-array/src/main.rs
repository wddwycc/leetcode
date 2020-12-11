pub struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut res = 0;
        for cur in 0..nums.len() {
            if cur == 0 {
                res += 1;
                continue;
            }
            let num = nums[cur];
            if num != nums[cur - 1] {
                nums[res] = num;
                res += 1;
            }
        }
        res as i32
    }
}

fn main() {
    println!("Hello, world!");
}
