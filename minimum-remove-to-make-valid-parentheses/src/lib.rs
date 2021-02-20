use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut idx_to_del = HashSet::new();
        let mut stack = vec![];

        for (idx, c) in s.char_indices() {
            match c {
                '(' => stack.push(idx),
                ')' => {
                    if stack.len() > 0 {
                        stack.pop();
                    } else {
                        idx_to_del.insert(idx);
                    }
                }
                _ => continue,
            }
        }
        for i in stack {
            idx_to_del.insert(i);
        }
        s.char_indices()
            .filter_map(|(idx, c)| {
                if idx_to_del.contains(&idx) {
                    None
                } else {
                    Some(c)
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
