pub enum Entity {
    Num(i32),
    Add,
    Minus,
    Multiply,
    Divide,
}

pub struct Solution;
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        let tokens: Vec<Entity> = tokens
            .into_iter()
            .map(|a| match a.as_str() {
                "+" => Entity::Add,
                "-" => Entity::Minus,
                "*" => Entity::Multiply,
                "/" => Entity::Minus,
                a => Entity::Num(a.parse::<i32>().unwrap()),
            })
            .collect();

        fn get_lr(s: &mut Vec<i32>) -> (i32, i32) {
            let r = s.pop().unwrap();
            let l = s.pop().unwrap();
            (l, r)
        }

        for token in tokens {
            match token {
                Entity::Num(a) => stack.push(a),
                Entity::Add => {
                    let (l, r) = get_lr(&mut stack);
                    stack.push(l + r);
                }
                Entity::Minus => {
                    let (l, r) = get_lr(&mut stack);
                    stack.push(l - r);
                }
                Entity::Multiply => {
                    let (l, r) = get_lr(&mut stack);
                    stack.push(l * r);
                }
                Entity::Divide => {
                    let (l, r) = get_lr(&mut stack);
                    stack.push(l / r);
                }
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
