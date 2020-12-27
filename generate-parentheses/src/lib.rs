pub struct Solution;
impl Solution {
    fn is_well_formed(src: &[char]) -> bool {
        let mut stack: Vec<&char> = vec![];
        for i in src {
            match stack.last() {
                Some(prev) => {
                    if *i == ')' && **prev == '(' {
                        stack.pop();
                    } else {
                        stack.push(i);
                    }
                }
                None => stack.push(i),
            }
        }
        stack.is_empty()
    }

    /// Backtracking
    ///
    /// # Arguements
    ///
    /// * to_consume: `(`'s count, `)`'s count
    fn dfs(prev: Vec<char>, to_consume: (usize, usize), res: &mut Vec<String>) {
        if to_consume.0 == 0 && to_consume.1 == 0 {
            if Self::is_well_formed(&prev) {
                res.push(prev.into_iter().collect())
            }
            return;
        }
        if to_consume.0 > 0 {
            let mut next = prev.clone();
            next.push('(');
            Self::dfs(next, (to_consume.0 - 1, to_consume.1), res);
        }
        if to_consume.1 > 0 {
            let mut next = prev.clone();
            next.push(')');
            Self::dfs(next, (to_consume.0, to_consume.1 - 1), res);
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut res = vec![];
        Self::dfs(vec![], (n, n), &mut res);
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
