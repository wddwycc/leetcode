pub struct Solution;
impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack: Vec<(char, usize)> = vec![];
        for c in s.chars() {
            if let Some(top) = stack.last_mut() {
                if top.0 == c {
                    if top.1 == (k as usize) - 1 {
                        stack.pop();
                        continue;
                    } else {
                        top.1 += 1;
                    }
                } else {
                    stack.push((c, 1));
                }
            } else {
                stack.push((c, 1));
            }
        }
        stack.into_iter().flat_map(|a| vec![a.0; a.1]).collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
