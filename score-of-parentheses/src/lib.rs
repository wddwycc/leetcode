pub struct Solution;
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack = vec![];
        stack.push(0);
        for c in s.chars() {
            if c == '(' {
                stack.push(0);
            }
            if c == ')' {
                let v = stack.pop().unwrap();
                let w = stack.pop().unwrap();
                stack.push(w + std::cmp::max(2 * v, 1));
            }
        }
        stack[0]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
