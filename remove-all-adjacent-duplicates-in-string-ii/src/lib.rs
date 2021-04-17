pub struct Solution;
impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack: Vec<(char, usize)> = vec![];
        for c in s.chars() {
            let same_c_top = match stack.last_mut() {
                Some(a) if a.0 == c => a,
                _ => {
                    stack.push((c, 1));
                    continue;
                }
            };
            if same_c_top.1 == (k as usize) - 1 {
                stack.pop();
            } else {
                same_c_top.1 += 1;
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
