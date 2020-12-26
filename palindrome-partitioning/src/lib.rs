pub struct Solution;
impl Solution {
    fn is_palindrome(src: &[char], mut start: usize, mut end: usize) -> bool {
        while start < end {
            if src[start] == src[end] {
                start += 1;
                end -= 1;
            } else {
                return false;
            }
        }
        true
    }

    fn dfs(src: &[char], start: usize, current_list: &Vec<String>, res: &mut Vec<Vec<String>>) {
        if start > src.len() - 1 {
            res.push(current_list.clone());
            return;
        }
        for i in start..src.len() {
            let head_is_palindrome = Self::is_palindrome(src, start, i);
            if head_is_palindrome {
                let next_current_list = {
                    let mut res = current_list.clone();
                    res.push(src[start..=i].iter().collect());
                    res
                };
                Self::dfs(src, i + 1, &next_current_list, res);
            }
        }
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        let chars = s.chars().collect::<Vec<_>>();
        if chars.len() == 0 {
            return vec![];
        }
        let mut res = vec![];
        Self::dfs(&chars, 0, &vec![], &mut res);
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
