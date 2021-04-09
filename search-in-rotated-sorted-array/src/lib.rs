pub struct Solution;
impl Solution {
    pub fn search(mut nums: Vec<i32>, target: i32) -> i32 {
        let offset = (|| {
            for i in 1..nums.len() {
                if nums[i] < nums[i - 1] {
                    return i;
                }
            }
            return 0;
        })();
        nums.rotate_left(offset);
        if let Ok(i) = nums.binary_search(&target) {
            return ((i + offset) % nums.len()) as i32;
        } else {
            return -1;
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
