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

    /// Backtracking dfs
    ///
    /// Constraint: consumed.0 must greater than consumed.1
    fn dfs(
        prev: Vec<char>,
        consumed_l: usize,
        consumed_r: usize,
        max: usize,
        res: &mut Vec<String>,
    ) {
        if consumed_l == max && consumed_r == max {
            if Self::is_well_formed(&prev) {
                res.push(prev.into_iter().collect())
            }
            return;
        }
        if consumed_l < max {
            let mut next = prev.clone();
            next.push('(');
            Self::dfs(next, consumed_l + 1, consumed_r, max, res);
        }
        if consumed_r < max && consumed_r < consumed_l {
            let mut next = prev.clone();
            next.push(')');
            Self::dfs(next, consumed_l, consumed_r + 1, max, res);
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut res = vec![];
        Self::dfs(vec![], 0, 0, n, &mut res);
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
