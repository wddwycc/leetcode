pub struct Solution;
impl Solution {
    pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        let mut res = 0;
        for av in a.iter() {
            for bv in b.iter() {
                for cv in c.iter() {
                    for dv in d.iter() {
                        if av + bv + cv + dv == 0 {
                            res += 1;
                        }
                    }
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
