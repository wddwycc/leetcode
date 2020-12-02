pub struct Solution {}
impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        let prev_k = (k - 1) / 2 + 1;
        match (Self::kth_grammar(n - 1, prev_k), k % 2) {
            (0, 0) => 1,
            (0, 1) => 0,
            (1, 0) => 0,
            (1, 1) => 1,
            _ => panic!(),
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
