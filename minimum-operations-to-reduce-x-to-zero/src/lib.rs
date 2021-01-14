pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let len = nums.len();
        let prefix_sum = {
            let mut res = vec![];
            let mut sum = 0;
            for a in nums.iter() {
                sum += a;
                res.push(sum);
            }
            res
        };
        // NOTE: transform the question into: find minimum subarray to sum up to target
        let target = prefix_sum[len - 1] - x;
        if target == 0 {
            return len as i32;
        }

        let mut res = None;
        for i in 0..len {
            for j in i..len {
                let v = {
                    if i == 0 {
                        prefix_sum[j]
                    } else {
                        prefix_sum[j] - prefix_sum[i - 1]
                    }
                };
                if v != target {
                    continue;
                }
                let distance = j + 1 - i;
                res = Some(res.map(|a| std::cmp::max(distance, a)).unwrap_or(distance));
            }
        }
        res.map(|a| (len - a) as i32).unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
