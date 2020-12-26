#[derive(Clone)]
pub enum State {
    NoAsk,
    Validated(bool),
}

// time: O()
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

    fn dfs(
        src: &[char],
        start: usize,
        current_list: &Vec<String>,
        res: &mut Vec<Vec<String>>,
        cache: &mut Vec<Vec<State>>,
    ) {
        if start > src.len() - 1 {
            res.push(current_list.clone());
            return;
        }
        for end in start..src.len() {
            let is_palindrome = {
                if let State::Validated(cached) = cache[start][end] {
                    cached
                } else {
                    let res = Self::is_palindrome(src, start, end);
                    cache[start][end] = State::Validated(res);
                    res
                }
            };
            if is_palindrome {
                let next_current_list = {
                    let mut res = current_list.clone();
                    res.push(src[start..=end].iter().collect());
                    res
                };
                Self::dfs(src, end + 1, &next_current_list, res, cache);
            }
        }
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        let chars = s.chars().collect::<Vec<_>>();
        let n = chars.len();
        if n == 0 {
            return vec![];
        }
        let mut res = vec![];
        let mut cache = vec![vec![State::NoAsk; n]; n]; // cache is_palindrome records
        Self::dfs(&chars, 0, &vec![], &mut res, &mut cache);
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
