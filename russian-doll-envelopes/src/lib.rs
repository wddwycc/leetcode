pub struct Solution;
impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_by(|x, y| match x[0].cmp(&y[0]) {
            std::cmp::Ordering::Equal => y[1].cmp(&x[1]),
            a => a,
        });
        let heights: Vec<i32> = envelopes.into_iter().map(|a| a[1]).collect();
        Self::longest_increasing_subsequence(&heights) as i32
    }

    fn longest_increasing_subsequence(nums: &[i32]) -> usize {
        if nums.len() <= 1 {
            return nums.len();
        }
        let mut res = vec![];
        res.push(nums[0]);
        for i in 1..nums.len() {
            match res.binary_search(&nums[i]) {
                Err(n) => {
                    if n >= res.len() {
                        res.push(nums[i]);
                    } else {
                        res[n] = nums[i];
                    }
                }
                _ => (),
            }
        }
        res.len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
