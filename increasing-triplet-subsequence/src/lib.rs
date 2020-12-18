use std::i32::MAX;
pub struct Solution;
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let (mut s, mut l) = (MAX, MAX);
        for n in nums {
            if n < s {
                s = n;
            } else if n > s && n < l {
                l = n;
            } else if n > s && n > l {
                return true;
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
