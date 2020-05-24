pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match stack.last() {
            Some(&a) => {
                let paired = match (a, c) {
                    ('{', '}') => true,
                    ('[', ']') => true,
                    ('(', ')') => true,
                    _ => false,
                };
                if paired {
                    stack.pop();
                } else {
                    stack.push(c);
                }
            }
            None => stack.push(c),
        }
    }
    stack.is_empty()
}

fn main() {
    println!("Hello, world!");
}
