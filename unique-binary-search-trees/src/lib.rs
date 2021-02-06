use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        // f(n) = f(0) * f(n - 1) + f(1) * f(n - 2) + ... f(n - 1) * f(0)
        // f(0) = 1 (bottom-case)

        // let's try top-down dp
        let mut cache = HashMap::new();
        Self::dfs(n, &mut cache)
    }

    fn dfs(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if let Some(a) = cache.get(&n) {
            return *a;
        }
        if n == 0 {
            return 1;
        }
        let mut res = 0;
        for a in 0..n {
            res += Self::dfs(a, cache) * Self::dfs(n - 1 - a, cache);
        }
        cache.insert(n, res);
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
