use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        Self::dfs_ct(m, n, 0, 0, &mut HashMap::new(), &mut HashMap::new()) as i32
    }

    fn dfs_ct(
        m: usize,
        n: usize,
        prev: u16,
        i: usize,
        next_col_cache: &mut HashMap<u16, Vec<u16>>,
        ct_cache: &mut HashMap<(u16, usize), usize>,
    ) -> usize {
        if let Some(cached) = ct_cache.get(&(prev, n - i)) {
            return *cached;
        }

        let next_cols = match next_col_cache.get(&prev) {
            Some(a) => a.clone(),
            None => {
                let mut res = vec![];
                Self::dfs(m, prev, 0, 0, &mut res);
                next_col_cache.insert(prev, res.clone());
                res
            }
        };
        if i + 1 == n {
            return next_cols.len();
        }
        let mut ans = 0;
        for col in next_cols {
            ans += Self::dfs_ct(m, n, col, i + 1, next_col_cache, ct_cache);
            ans %= 1_000_000_007;
        }
        ct_cache.insert((prev, n - i), ans);
        return ans;
    }

    fn dfs(m: usize, prev: u16, i: usize, acc: u16, res: &mut Vec<u16>) {
        if i == m {
            res.push(acc);
            return;
        }

        let mut r = true;
        let mut g = true;
        let mut b = true;

        let v = (prev >> (i * 2)) & 0b11;
        if v == 1 {
            r = false;
        }
        if v == 2 {
            g = false;
        }
        if v == 3 {
            b = false;
        }
        if i > 0 {
            let v = acc >> ((i - 1) * 2) & 0b11;
            if v == 1 {
                r = false;
            }
            if v == 2 {
                g = false;
            }
            if v == 3 {
                b = false;
            }
        }
        if r {
            Self::dfs(m, prev, i + 1, acc | 1 << i * 2, res);
        }
        if g {
            Self::dfs(m, prev, i + 1, acc | 2 << i * 2, res);
        }
        if b {
            Self::dfs(m, prev, i + 1, acc | 3 << i * 2, res);
        }
    }
}
