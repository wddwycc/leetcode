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
    /// # Constraint:
    /// consumed.0 must greater than consumed.1
    ///
    /// # Arguements
    ///
    /// * consumed: `(`'s count, `)`'s count
    fn dfs(prev: Vec<char>, consumed: (usize, usize), max: usize, res: &mut Vec<String>) {
        if consumed.0 == max && consumed.1 == max {
            if Self::is_well_formed(&prev) {
                res.push(prev.into_iter().collect())
            }
            return;
        }
        if consumed.0 < max {
            let mut next = prev.clone();
            next.push('(');
            Self::dfs(next, (consumed.0 + 1, consumed.1), max, res);
        }
        if consumed.1 < max && consumed.1 < consumed.0 {
            let mut next = prev.clone();
            next.push(')');
            Self::dfs(next, (consumed.0, consumed.1 + 1), max, res);
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut res = vec![];
        Self::dfs(vec![], (0, 0), n, &mut res);
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
