pub struct Solution;
impl Solution {
    pub fn min_absolute_sum_diff(mut nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let merged_nums: Vec<(i32, i32)> = (0..nums1.len()).map(|i| (nums1[i], nums2[i])).collect();
        nums1.sort();
        // binary search to find the larget offset
        let mut offset = 0;
        for (x, y) in merged_nums.iter() {
            let mut res = (x - y).abs();
            match nums1.binary_search(&y) {
                Ok(_) => res = 0,
                Err(i) => {
                    if i < merged_nums.len() {
                        res = res.min((nums1[i] - y).abs());
                    }
                    if i > 0 {
                        res = res.min((nums1[i - 1] - y).abs());
                    }
                }
            }
            offset = offset.max((x - y).abs() - res);
        }
        let mut res: i64 = merged_nums
            .into_iter()
            .map(|(x, y)| (x - y).abs() as i64)
            .sum();
        res -= offset as i64;
        res %= 10_i64.pow(9) + 7;
        res as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
