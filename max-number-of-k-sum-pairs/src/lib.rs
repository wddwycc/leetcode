use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let mut hashmap = HashMap::new();
        for num in nums {
            let target = k - num;
            match hashmap.get_mut(&target) {
                Some(a) => {
                    if *a > 1 {
                        *a -= 1;
                    } else {
                        hashmap.remove_entry(&target);
                    }
                    res += 1;
                }
                None => {
                    *hashmap.entry(num).or_insert(0) += 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
