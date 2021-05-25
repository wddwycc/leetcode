pub struct Solution;
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for token in tokens {
            if let Ok(v) = token.parse::<i32>() {
                stack.push(v);
            } else {
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                match token.as_str() {
                    "+" => stack.push(l + r),
                    "-" => stack.push(l - r),
                    "*" => stack.push(l * r),
                    "/" => stack.push(l / r),
                    _ => panic!(),
                }
            }
        }
        stack[0]
    }
}
