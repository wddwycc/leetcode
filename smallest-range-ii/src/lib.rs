pub struct Solution;
impl Solution {
    pub fn smallest_range_ii(mut a: Vec<i32>, k: i32) -> i32 {
        if a.len() == 1 {
            return 0;
        }
        a.sort();
        let mut res = std::i32::MAX;
        for i in 0..(a.len() - 1) {
            //  - - - i || - - - (n)
            let min = std::cmp::min(a[0] + k, a[i + 1] - k);
            let max = std::cmp::max(a[i] + k, a[a.len() - 1] - k);
            res = std::cmp::min(res, max - min);
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
