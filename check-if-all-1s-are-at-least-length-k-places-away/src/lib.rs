pub struct Solution;
impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut last_one_idx = None;
        for i in 0..nums.len() {
            if nums[i] != 1 {
                continue;
            }
            if let Some(last) = last_one_idx {
                if i - 1 - last < k as usize {
                    return false;
                }
            }
            last_one_idx = Some(i);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
