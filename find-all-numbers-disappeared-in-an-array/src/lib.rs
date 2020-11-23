pub struct Solution {}
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![];
        }
        let mut nums = nums;
        nums.sort();
        let mut prev = 0;
        let mut res = vec![];
        for num in nums.iter() {
            while *num > prev + 1 {
                res.push(prev + 1);
                prev += 1
            }
            prev = *num;
        }
        let mut distance = nums.len() as i32 - nums[nums.len() - 1];
        while distance > 0 {
            res.push(nums.len() as i32 - distance + 1);
            distance -= 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
        assert_eq!(
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 7, 2, 3, 1]),
            vec![5, 6, 8]
        );
        assert_eq!(Solution::find_disappeared_numbers(vec![1, 1]), vec![2]);
        assert_eq!(Solution::find_disappeared_numbers(vec![1, 2]), vec![]);
        assert_eq!(Solution::find_disappeared_numbers(vec![]), vec![]);
    }
}
