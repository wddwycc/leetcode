pub struct Solution;
impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let max_l = {
            let mut res = vec![];
            let mut max = 0;
            for &num in nums.iter() {
                max = max.max(num);
                res.push(max);
            }
            res
        };
        let min_r = {
            let mut res = vec![];
            let mut min = std::i32::MAX;
            for &num in nums.iter().rev() {
                min = min.min(num);
                res.push(min);
            }
            res.reverse();
            res
        };
        for i in 0..(nums.len() - 1) {
            if max_l[i] <= min_r[i + 1] {
                return (i + 1) as i32;
            }
        }
        panic!()
    }
}
