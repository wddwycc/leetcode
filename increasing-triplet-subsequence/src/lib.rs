pub struct Solution;
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        Self::helper(&nums)
    }

    pub fn helper(nums: &[i32]) -> bool {
        let mut stack = vec![];
        for (idx, n) in nums.iter().enumerate() {
            match stack.len() {
                0 => stack.push(n),
                1 => {
                    let prev = stack[0];
                    if n > prev {
                        stack.push(n)
                    } else {
                        stack.pop();
                        stack.push(n);
                    }
                }
                2 => {
                    if n > stack[1] {
                        return true;
                    }
                    if n > stack[0] && n < stack[1] {
                        stack.pop();
                        stack.push(n);
                    }
                    if n < stack[0] {
                        if Self::helper(&nums[idx..]) {
                            return true;
                        }
                    }
                }
                _ => panic!(),
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
