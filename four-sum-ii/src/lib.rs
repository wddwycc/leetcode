use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        let mut res = 0;

        let ab_sum = {
            let mut sum = HashMap::new();
            for i in &a {
                for j in &b {
                    *sum.entry(i + j).or_insert(0) += 1;
                }
            }
            sum
        };
        for i in &c {
            for j in &d {
                if let Some(v) = ab_sum.get(&(0 - i - j)) {
                    // TODO
                    res += v;
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
