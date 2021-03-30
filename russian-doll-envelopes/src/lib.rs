pub struct Solution;
impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort();
        let heights = {
            let mut res = vec![];
            let mut width = envelopes[0][0];
            let mut heights = vec![envelopes[0][1]];
            for envelope in &envelopes[1..] {
                if envelope[0] == width {
                    heights.push(envelope[1]);
                } else {
                    heights.reverse();
                    res.append(&mut heights);
                    width = envelope[0];
                    heights = vec![envelope[1]];
                }
            }
            heights.reverse();
            res.append(&mut heights);
            res
        };
        // TODO: improve sorting
        Self::longest_increasing_subsequence(&heights)
    }

    fn longest_increasing_subsequence(nums: &[i32]) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
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
        res.len() as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
