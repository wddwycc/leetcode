pub struct Solution;
impl Solution {
    // time: O(n), space: O(n)
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        // NOTE: calc left bound
        let l_bound = {
            let mut res = nums.len() - 1;
            let mut stack: Vec<usize> = vec![];
            let mut cur = 0;
            while cur < nums.len() {
                if stack.is_empty() || nums[cur] >= nums[*stack.last().unwrap()] {
                    stack.push(cur);
                    cur += 1;
                } else {
                    res = res.min(stack.pop().unwrap());
                }
            }
            res
        };
        // NOTE: calc right bound
        let r_bound = {
            let mut res = 0;
            let mut stack: Vec<usize> = vec![];
            let mut cur = nums.len() - 1;
            loop {
                if stack.is_empty() || nums[cur] <= nums[*stack.last().unwrap()] {
                    stack.push(cur);
                    if cur == 0 {
                        break;
                    }
                    cur -= 1;
                } else {
                    res = res.max(stack.pop().unwrap());
                }
            }
            res
        };
        if r_bound > l_bound {
            (r_bound - l_bound + 1) as i32
        } else {
            0
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
