pub struct Solution;
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        if nums.len() == 1 {
            return 1;
        }
        let mut cur = 1;
        let mut res = 0;
        let mut tmp = 1;
        while cur < nums.len() {
            if nums[cur - 1] >= nums[cur] {
                if tmp > res {
                    res = tmp;
                }
                tmp = 1;
            } else {
                tmp += 1;
            }
            cur += 1
        }
        if tmp > res {
            res = tmp;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
    }
}
