use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn cached_kth_grammar(n: i32, k: i32, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
        if let Some(cached) = cache.get(&(n, k)) {
            return *cached;
        }
        let res = {
            if n == 1 {
                0
            } else {
                let prev_k = (k - 1) / 2 + 1;
                match (Self::cached_kth_grammar(n - 1, prev_k, cache), k % 2) {
                    (0, 0) => 1,
                    (0, 1) => 0,
                    (1, 0) => 0,
                    (1, 1) => 1,
                    _ => panic!(),
                }
            }
        };
        cache.insert((n, k), res);
        res
    }

    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let mut cache = HashMap::new();
        Self::cached_kth_grammar(n, k, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
